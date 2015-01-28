use config::Config;
use hyparview::messages::{HyParViewMessage,Serializable,Disconnect,ForwardJoin,Join,JoinAck,NeighborRequest,NeighborResponse,Priority,Result,Shuffle,ShuffleReply};
use log::set_logger;
use logger::LocalLogger;
use std::io::Timer;
use std::io::net::ip::SocketAddr;
use std::io::net::tcp::TcpStream;
use std::rand;
use std::sync::{Arc,RwLock};
use std::time::Duration;
use std::thread::{Builder};
use std::vec::Vec;
use std::sync::mpsc::{channel,Receiver,Sender};

pub mod messages;

pub trait Shipper {
    fn ship(&self, msg: &Serializable, dest: &SocketAddr) -> bool;
}

pub struct SocketShipper {
    local_addr: SocketAddr,
}
impl Shipper for SocketShipper {
    fn ship(&self, msg: &Serializable, dest: &SocketAddr) -> bool {
        let mut success = false;
        match TcpStream::connect_timeout(*dest, Duration::seconds(4)) {
            Ok(ref mut socket) => {
                match msg.serialize(&mut* socket, &self.local_addr) {
                    Ok(_) => success = true,
                    Err(e) => warn!("failed to send message to peer {}: {}", dest, e),
                }
            },
            Err(e) => warn!("failed to open connection to peer (to forward the shuffle): {}", e),
        }
        success
    }
}

pub struct HyParViewContext<'a> {
    config: Arc<Config>,

    // NOTE: not sure I'm really doing the best thing here using RwLock, but it's allowing me to mutate the Vec, so I think I'm on the right path there.
    // however, I don't think i can retain a mutable reference to the open, outbound tcp connection (with the socket addr). thus, am punting on it for now..
    active_view: Box<Vec<SocketAddr>>,
    passive_view: Box<Vec<SocketAddr>>,
    shipper: Box<Shipper + 'a>,
}
impl<'a> HyParViewContext<'a> {
    fn new(config: Arc<Config>) -> HyParViewContext<'a> {
        let c = config.clone();
        HyParViewContext { 
            config: config,
            active_view: Box::new(Vec::with_capacity(c.active_view_size)),
            passive_view: Box::new(Vec::with_capacity(c.passive_view_size)),
            shipper: Box::new(SocketShipper { local_addr: c.local_addr }),
        }
    }

    /// central event dispatcher for the hyparview module
    pub fn listen(&mut self, receiver: Receiver<HyParViewMessage>) {
        loop {
            // TODO: try_recv() does *not* block, and might be nice for a gentle shutdown of the listener
            match receiver.recv().unwrap() {
                HyParViewMessage::JoinBegin => self.join(),
                HyParViewMessage::JoinMessage(_,addr) => self.handle_join(&addr),
                HyParViewMessage::ForwardJoinMessage(msg,addr) => self.handle_forward_join(&msg, &addr),
                HyParViewMessage::JoinAckMessage(_,addr) => self.handle_join_ack(&addr),
                HyParViewMessage::DisconnectMessage(_,addr) => self.handle_disconnect(&addr),
                HyParViewMessage::NeighborRequestMessage(msg,addr) => self.handle_neighbor_request(&msg, &addr),
                HyParViewMessage::NeighborResponseMessage(msg,addr) => self.handle_neighbor_response(&msg, &addr),
                HyParViewMessage::ShuffleMessage(msg,addr) => self.handle_shuffle(msg, &addr),
                HyParViewMessage::ShuffleReplyMessage(msg,_) => self.handle_shuffle_reply(&msg),
                HyParViewMessage::NextShuffleRound => self.handle_next_shuffle_round(),
                HyParViewMessage::PeerDisconnect(addr) => self.handle_peer_failure(&addr),
            }
        }
    }

    /// call an arbitrary contact_node and send a 'JOIN' message
    pub fn join(&self) {
        debug!("in join()");
        // TODO: add callback to ensure we got a response (Neighbor) msg from some peer
        let filtered: Vec<SocketAddr> = self.config.contact_nodes.iter().filter_map(|&x| if x.ne(&self.config.local_addr) {Some(x)} else {None}).collect();

        match select_random(&filtered) {
            Some(node) => {
                debug!("sending join request to {}", node);
                self.shipper.ship(&Join::new(), node);
             },
            None => info!("no unique contact node addresses available"),
        }
    }

    /// a contact_node (or really any node, for that matter) receives a JOIN request from a node that wants to join the cluster.
    /// the requesting could be a completely new node, or it could be a node that bounced or possibly it resent the JOIN request
    /// due to timeout (because it didn't receive a response).
    fn handle_join(&mut self, sender: &SocketAddr) {
        debug!("in handle_join for sender {}", sender);
        if sender.eq(&self.config.local_addr) {
            warn!("something funky just happened: this node (a contact node) got a join request from itself, or another claiming the same IP:port");
            return;
        }

        let forward_join = ForwardJoin::new(sender, self.config.active_random_walk_length, self.config.passive_random_walk_length, self.config.active_random_walk_length);
        if self.active_view.len() == 0 {
            info!("adding joining node to this node's active_view: {}", &sender);
            self.add_to_active_view(sender);
            info!("about to ship join ack to: {}", &sender);
            self.shipper.ship(&JoinAck::new(), sender);
        } else {
            for peer in self.active_view.iter() { 
                info!("sending forward join to {} for {}", peer, &sender);
                self.shipper.ship(&forward_join, peer);
            }
        }
    }

    fn handle_forward_join(&mut self, msg: &ForwardJoin, sender: &SocketAddr) {
        debug!("in handle_forward_join for {} from {}, ttl: {}", msg.originator, sender, msg.ttl);
        // possibly take the request, unless the current node is the one that originated it (damn data races in distributed systems!)
        if (self.active_view.len() <= 1 || msg.ttl <= 0) && msg.originator.ne(&self.config.local_addr) {
            info!("adding forwarded joining node to active_view: {}", &msg.originator);
            self.add_to_active_view(&msg.originator);
            self.shipper.ship(&JoinAck::new(), &msg.originator);
            return;
        } 

        let mut ttl = msg.ttl - 1;
        if msg.ttl == msg.prwl {
            if msg.originator.ne(&self.config.local_addr) {
                self.add_to_passive_view(&msg.originator);
            } else { 
                // bump the ttl so that when we forward the message, the originator can attempt to be added into the passive view on another node
                ttl += 1;
            }
        }

        let forward_join = ForwardJoin::new(&msg.originator, msg.arwl, msg.prwl, ttl);
        let filtered: Vec<SocketAddr> = self.active_view.iter().filter_map(|&x| if x.ne(sender) && x.ne(&msg.originator) {Some(x)} else {None}).collect();
        let peer = match select_random(&filtered) {
            Some(p) => p,
            None => &msg.originator, //if no other peers, just send back to the caller
        };
        debug!("sending next forward_join for {} to {}, ttl: {}", msg.originator, peer, ttl);
        self.shipper.ship(&forward_join,peer);
    }

    fn add_to_active_view(&mut self, peer: &SocketAddr) {
        if self.config.local_addr.eq(peer) {
            warn!("attempting to add a node to it's own active list. ignoring");
            return;
        }

        // add to the active list if node is not already in it
        if self.active_view.iter().find(|&x| x.eq(peer)).is_none() {
            debug!("adding peer to active view: {}", peer);
            self.active_view.push(*peer);

            while self.active_view.len() > self.config.active_view_size {
                let removed = self.active_view.remove(0);
                self.add_to_passive_view(&removed);
                debug!("remove random node from active_list: {}", removed);
                self.shipper.ship(&Disconnect::new(), &removed);
            }
        }

        // check if the node is in the passive view, and remove it
        // also, we should never get into the situation where a node is in both the active and passive lists
        // but having this as a separate check helps prevent against it (argh, fucking data races)
        let idx = self.passive_view.iter().position(|&x| x.eq(peer));
        if idx.is_some() {
            self.passive_view.remove(idx.unwrap());
        }
    }

    fn add_to_passive_view(&mut self, peer: &SocketAddr) {
        if self.config.local_addr.eq(peer) || self.passive_view.iter().find(|&x| x.eq(peer)).is_some() {
            return;
        }

        self.passive_view.push(*peer);
        while self.passive_view.len() > self.config.passive_view_size {
            self.passive_view.remove(0);
        }
    }

    fn handle_join_ack(&mut self, sender: &SocketAddr) {
        debug!("received join_ack from {}", sender);
        self.add_to_active_view(sender);
    }

    fn handle_disconnect(&mut self, sender: &SocketAddr) {
        debug!("received disconnect from {}", sender);
        // remove from active list, if currently in it
        let idx = self.active_view.iter().position(|&x| x.eq(sender));
        if idx.is_some() {
            self.active_view.remove(idx.unwrap());

            // add to the passive list, if not in it (either due to data race or programming bug)
            if self.passive_view.iter().find(|&x| x.eq(sender)).is_some() {
                self.passive_view.push(*sender);
            }
            while self.passive_view.len() >= self.config.passive_view_size {
                self.passive_view.remove(0);
            }
        }

        self.send_neighbor_request(None);
    }

    // this is a bit optimistic in that after we successfully send the NIEGHBORBOR msg, we expect to get some response (async, of course).
    // we should keep around some timeout reference so we can try another peer (and possibly kick out the one that timed out), but that's for the future.
    fn send_neighbor_request(&mut self, last_addr: Option<&SocketAddr>) {
        // use of this index is a bit ... optimistic/interesting. not sure i like it, but it's a loose way to asynchrounously iterate through the passive_view
        let mut idx = match last_addr {
            None => 0,
            Some(addr) => match self.passive_view.iter().position(|&x| x.eq(addr)) {
                Some(i) => i,
                None => 0,
            }
        };

        // bail out if we've already cycled through the passive_view, unless the active view is now empty
        if idx >= self.passive_view.len() {
            if self.active_view.len() == 0 {
                idx = 0
            } else {
                return;
            }
        }

        let priority = match self.active_view.len() {
            0 => Priority::High,
            _ => Priority::Low,
        };

        // note: this is suboptimal as SimpleSender will block for each send (thus blocking the entire
        // hyparview event processing thread).
        // TODO: (maybe) add callback handler in case peer never responds (so we can all another). however, if that does trigger,
        // we should check if we've got a full active_view, and not bother with another NEIGHBOR request, in that case.
        for i in range(idx, self.passive_view.len()) {
            let neighbor = NeighborRequest::new(priority);
            if self.shipper.ship(&neighbor, &self.passive_view[i]) {
                return;
            } else {
                self.passive_view.remove(i);
            }
            idx += 1;
        }
    }

    fn handle_neighbor_request(&mut self, msg: &NeighborRequest, sender: &SocketAddr) {
        if Priority::Low.eq(&msg.priority) && self.active_view.len() == self.config.active_view_size {
            let resp = NeighborResponse::new(Result::Reject);
            self.shipper.ship(&resp, sender);
            return;
        }

        self.add_to_active_view(sender);
        let resp = NeighborResponse::new(Result::Accept);
        self.shipper.ship(&resp, sender);
    }

    fn handle_neighbor_response(&mut self, msg: &NeighborResponse, sender: &SocketAddr) {
        match msg.result {
            Result::Accept => self.add_to_active_view(sender),
            Result::Reject => {
                // check if we've gotten requests (neighbor or shuffle) from other nodes and now the active_view is full
                if self.active_view.len() < self.config.active_view_size {
                    self.send_neighbor_request(Some(sender));
                }
            },
        };
    }

    fn handle_next_shuffle_round(&mut self) {
        debug!("start of next shuffle round: active_view {:?}, passive_view {:?}", &*self.active_view, &*self.passive_view);
        match self.active_view.len() {
            0 => self.join(),
            _ => self.do_shuffle(),
        }
    }

    fn do_shuffle(&mut self) {
        debug!("in do_shuffle()");
        // let target_addr = select_random(&*self.active_view);

        // let active_filtered: Vec<&SocketAddr> = self.active_view.iter().filter(|&x| x.ne(target_addr) && x.ne(&self.config.local_addr)).collect();
        // let passive_filtered: Vec<&SocketAddr> = self.passive_view.iter().filter(|&x| x.ne(target_addr) && x.ne(&self.config.local_addr)).collect();

        // let nodes = self.build_shuffle_list(&active_filtered, &passive_filtered);
        // let shuffle = Shuffle::new(self.config.local_addr, nodes, self.config.shuffle_walk_length);
        // self.shipper.ship(&shuffle, target_addr);

        // // additionally, possibly send shuffle message to a contact_node
        // let rand: usize = rand::random();
        // if rand % 10 == 0  {
        //     let filtered: Vec<&SocketAddr> = self.config.contact_nodes.iter().filter(|&x| x.ne(target_addr) && x.ne(&self.config.local_addr)).collect();
        //     if filtered.len() > 0 {
        //         let addr = select_random(&filtered);
        //         self.shipper.ship(&shuffle, *addr);
        //     }
        // }
        debug!("end do_shuffle()");
    }

    //fn build_shuffle_list<'b>(&'b self, active_view: &Vec<&'b SocketAddr>) -> Vec<&'b SocketAddr> {
        // let active_cnt = self.config.shuffle_active_view_count as usize;
        // let passive_cnt = self.config.shuffle_passive_view_count as usize;
        // let mut nodes: Vec<&'b SocketAddr> = Vec::with_capacity(1us + active_cnt + passive_cnt);
        // nodes.push(&self.config.local_addr);
        
        // select_multiple_random(&*active_view, &mut nodes, active_cnt);
        // // TODO: filter the passive_view with the nodes vector
        // select_multiple_random(&*self.passive_view, &mut nodes, passive_cnt);
        // nodes
    //}

    fn handle_shuffle(&mut self, msg: Shuffle, sender: &SocketAddr) {
        debug!("in handle_shuffle");
        // { 
        //     let mut to_avoid: Vec<SocketAddr> = Vec::with_capacity(3);
        //     to_avoid.push(*sender);
        //     // note this clone() is kind of a hack to get around the borrow checker ... someday i'll make this better 
        //     let local = self.config.local_addr.clone();
        //     to_avoid.push(local);
        //     to_avoid.push(msg.originator);
        //     let active_filtered = filter(&*self.active_view, &to_avoid);

        //     // determine if this node should handle the request or pass it on down
        //     if msg.ttl > 0 && self.active_view.len() > 1 {
        //         debug!("in handle_shuffle, going to forward the request from {} on behalf of {}", sender, msg.originator);
        //         let addr: &SocketAddr = match select_random(&active_filtered) {
        //             Some(addr) => *addr,
        //             None => {
        //                 debug!("could not get a random peer from the active list, so just sending shuffle msg to caller");
        //                 &msg.originator
        //             },
        //         };
        //         let shuffle = Shuffle::new(msg.originator, msg.nodes, msg.ttl - 1);
        //         self.shipper.ship(&shuffle, addr);
        //         return;
        //     }

        //     let passive_filtered = filter(&*self.passive_view, &to_avoid);
        //     let nodes = self.build_shuffle_list(&active_filtered, &passive_filtered);
            
        //     // NOTE: the clone() of the msg.nodes vec is a hack - clean up the ownership someday!!!
        //     let shuffle_reply = ShuffleReply::new(msg.nodes.clone(), nodes);
        //     self.shipper.ship(&shuffle_reply, &msg.originator);
        // }

        // let empty_vec: Vec<SocketAddr> = Vec::new();
        // self.apply_shuffle(&msg.nodes, &empty_vec);
    }


    fn apply_shuffle(&mut self, nodes: &Vec<SocketAddr>, filter: &Vec<SocketAddr>) {
        // let mut filter_idx = 0;
        // let filter_len = filter.len();

        // for node in nodes.iter() {
        //     // check to see if node is in active_view or passive_view - skip node if it is
        //     if self.active_view.iter().find(|&x| x.eq(node)).is_some() || self.passive_view.iter().find(|&x| x.eq(node)).is_some() {
        //         continue;
        //     }

        //     // if passive_view is at limit, remove one of the nodes that we sent over to the peer, as ref'd in the filter array; 
        //     // remove or a random node is filter is exhausted.
        //     while self.passive_view.len() >= self.config.passive_view_size - 1 {
        //         if filter_len > 0 && filter_idx < filter_len {
        //             let cur = filter[filter_idx];
        //             filter_idx += 1;
        //             let idx = self.passive_view.iter().position(|&x| x.eq(&cur));
        //             if idx.is_some() {
        //                 self.passive_view.remove(idx.unwrap());
        //             } else {
        //                 continue;
        //             }
        //         } else {
        //             self.passive_view.remove(0);
        //         }
        //     }
        //     self.passive_view.push(*node);
        // }
    }

    fn handle_shuffle_reply(&mut self, msg: &ShuffleReply) {
        debug!("in handle_shuffle_reply");
        self.apply_shuffle(&msg.nodes, &msg.sent_nodes);
    }
        
    fn handle_peer_failure(&mut self, addr: &SocketAddr){
        // remove from active list, if currently in it
        let idx = self.active_view.iter().position(|&x| x.eq(addr));
        if idx.is_some() {
            debug!("detected failed peer {}", addr);
            self.active_view.remove(idx.unwrap());
        }

        self.send_neighbor_request(None);
    }
}

// pub fn select_multiple_random<T: PartialEq>(src: &Vec<T>, dest: &mut Vec<T>, cnt: usize) {
//     if src.len() <= cnt {
//         for addr in src.iter() {
//             if dest.iter().find(|&x| x.eq(addr)).is_none() {
//                 dest.push(*addr);
//             }
//         }
//         return;
//     }

//     let mut c = 0;
//     while c < cnt {
//         match select_random(src) {
//             Some(addr) => {
//                 if dest.iter().find(|&x| x.eq(addr)).is_none() { 
//                     dest.push(*addr);
//                     c += 1;
//                 }
//             },
//             None => break,
//         }
//     }
// }

pub fn select_random<T>(v: &Vec<T>) -> Option<&T> {
    if v.len() == 0 {
        return None;
    }
    let rand: usize = rand::random();
    let idx = rand % v.len();
    Some(&v[idx])
}

// note: you can more easily do this with a function & filter(), but if you need a vec, not an iterator,
// you would to rip through the iterator to build that vec - thus yielding O(2n).
// fn filter<'a, T: PartialEq>(v: &'a Vec<T>, filter: &Vec<T>) -> Vec<&'a T> {
//     let mut filtered = Vec::with_capacity(v.len());
//     for t in v.iter() {
//         if filter.iter().find(|&x| x.eq(t)).is_none() {
//             filtered.push(t);
//         }
//     }
//     return filtered;
// }

fn timed_shuffle(sender: Sender<HyParViewMessage>) {
    let mut timer = Timer::new().unwrap();
    let periodic = timer.periodic(Duration::seconds(4));
    loop {
        periodic.recv().unwrap();
        match sender.send(HyParViewMessage::NextShuffleRound) {
            Ok(_) => {},
            Err(e) => info!("received an erro while trying to send message to begin next shuffle round"),
        }
    };
}

pub fn start_service(config: Arc<Config>) -> Sender<HyParViewMessage> {
    info!("starting up hyparview");
    let (sender, receiver) = channel::<HyParViewMessage>();

    let config_clone = config.clone();
    Builder::new().name("hpv-event".to_string()).spawn(move || {
        set_logger(Box::new(LocalLogger::new()));
        HyParViewContext::new(config_clone).listen(receiver);
    });

    sender.send(HyParViewMessage::JoinBegin);

    let sender_clone = sender.clone();
    Builder::new().name("hpv-timer".to_string()).spawn(move || {
        timed_shuffle(sender_clone);
    });

    sender.clone()
}

mod tests {
    use std::io::net::ip::{SocketAddr};
    use super::*;

}
