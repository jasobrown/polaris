#![feature(slicing_syntax)]

use config::Config;
use hyparview::messages::{HyParViewMessage,Disconnect,ForwardJoin,Join,JoinAck,NeighborRequest,NeighborResponse,Priority,Result,Shuffle,ShuffleReply};
use std::io::Timer;
use std::io::net::ip::SocketAddr;
use std::io::net::tcp::TcpStream;
use std::io::timer::sleep;
use std::rand;
use std::sync::{Arc,RWLock};
use std::time::Duration;
use std::thread::Thread;
use std::vec::Vec;
use std::sync::mpsc::Receiver;

pub mod messages;

pub struct HyParViewContext {
    config: Arc<Config>,

    // NOTE: not sure I'm really doing the best thing here using RWLock, but it's allowing me to mutate the Vec, so I think I'm on the right path there.
    // however, I don't think i can retain a mutable reference to the open, outbound tcp connection (with the socket addr). thus, am punting on it for now..
    active_view: RWLock<Vec<SocketAddr>>,
    passive_view: RWLock<Vec<SocketAddr>>
}
impl HyParViewContext {
    fn new(config: Arc<Config>) -> HyParViewContext {
        let c = config.clone();
        HyParViewContext { 
            config: config,
            active_view: RWLock::new(Vec::with_capacity(c.active_view_size)),
            passive_view: RWLock::new(Vec::with_capacity(c.passive_view_size)),
        }
    }

    /// call a random contact_node and send a 'JOIN' message
    pub fn join(&self) {
        println!("in join()");
       // TODO: add callback to ensure we got a response (Neighbor) msg from some peer
        let contact_nodes = &self.config.contact_nodes;

         if contact_nodes.len() == 0 || contact_nodes.len() == 1 && contact_nodes[0].eq(&self.config.local_addr) {
            println!("no unique contact node addresses available");
            return;
        }

        let rand: uint = rand::random();
        let idx = rand % contact_nodes.len();
        let node = contact_nodes[idx];
        println!("sending join request to {}", node);
        let msg = Join::new();
        let mut socket = TcpStream::connect(node).ok().expect("failed to open connection to peer");
        msg.serialize(&mut socket, &self.config.local_addr);
    }

    pub fn next_round(&self) {
        let mut timer = Timer::new().unwrap();
        let periodic = timer.periodic(Duration::seconds(10));
        loop {
            periodic.recv().unwrap();
            println!("start of next shuffle round, active_view len = {}", self.active_view.read().unwrap().len());
            match self.active_view.read().unwrap().len() {
                0 => self.join(),
                _ => self.do_shuffle(),
            }
        }
    }

    fn do_shuffle(&self) {
        let active_view = self.active_view.read().unwrap();
        let target_addr = HyParViewContext::select_random(&*active_view);
        let nodes = self.build_shuffle_list(&*active_view, &target_addr);
        let shuffle = Shuffle::new(self.config.local_addr, nodes, self.config.shuffle_walk_length);
        let mut socket = TcpStream::connect(target_addr).ok().expect("failed to open connection to peer");
        shuffle.serialize(&mut socket, &self.config.local_addr);

        // additionally, (TODO: probablistically) choose to send to a contact_node
        let addr = HyParViewContext::select_random(&self.config.contact_nodes); 
        let mut socket = TcpStream::connect(addr).ok().expect("failed to open connection to peer");
        shuffle.serialize(&mut socket, &self.config.local_addr);
    }

    fn build_shuffle_list(&self, active_view: &Vec<SocketAddr>, target_addr: &SocketAddr) -> Vec<SocketAddr> {
        let active_cnt = self.config.shuffle_active_view_count as uint;
        let mut nodes = Vec::with_capacity(1u + active_cnt + self.config.shuffle_passive_view_count as uint);
        nodes.push(self.config.local_addr);
        nodes.push(*target_addr);

        HyParViewContext::select_multiple_random(&*active_view, &mut nodes, 1u + active_cnt);
        let passive_view = self.passive_view.read().unwrap();
        HyParViewContext::select_multiple_random(&*passive_view, &mut nodes, 1u + active_cnt);
        nodes
    }

    fn select_multiple_random(src: &Vec<SocketAddr>, dest: &mut Vec<SocketAddr>, cnt: uint) {
        let mut c = 0;
        while c < cnt {
            let addr = HyParViewContext::select_random(src);
            let idx = HyParViewContext::index_of(src, &addr);
            if idx.is_none() {
                dest.push(addr);
                c += 1;
            }
        }
    }

    fn select_random(addrs: &Vec<SocketAddr>) -> SocketAddr {
        let rand: uint = rand::random();
        let idx = rand % addrs.len();
        addrs[idx]
    }

    pub fn listen(&self, rx: Receiver<HyParViewMessage>) {
        loop {
            // TODO: try_recv() does *not* block, and might be nice for a gentle shutdown of the listener
            match rx.recv().unwrap() {
                HyParViewMessage::JoinMessage(msg,addr) => self.handle_join(&msg,&addr),
                HyParViewMessage::ForwardJoinMessage(msg,addr) => self.handle_forward_join(&msg, &addr),
                HyParViewMessage::JoinAckMessage(_,addr) => self.handle_join_ack(&addr),
                HyParViewMessage::DisconnectMessage(_,addr) => self.handle_disconnect(&addr),
                HyParViewMessage::NeighborRequestMessage(msg,addr) => self.handle_neighbor_request(&msg, &addr),
                HyParViewMessage::NeighborResponseMessage(msg,addr) => self.handle_neighbor_response(&msg, &addr),
                HyParViewMessage::ShuffleMessage(msg,addr) => self.handle_shuffle(msg, &addr),
                HyParViewMessage::ShuffleReplyMessage(msg,addr) => self.handle_shuffle_reply(&msg, &addr),
            }
        }
    }

    /// a contact_node (or really any node, for that matter) receives a JOIN request from a node that wants to join the cluster.
    /// the requesting could be a completely new node, or it could be a node that bounced or possibly it resent the JOIN request
    /// due to timeout (because it didn't receive a response).
    fn handle_join(&self, msg: &Join, sender: &SocketAddr) {
        println!("in handle_join");
        let forward_join = ForwardJoin::new(sender, self.config.active_random_walk_length, self.config.passive_random_walk_length, self.config.active_random_walk_length);
        for peer in self.active_view.read().unwrap().iter() { 
            println!("sending forward join to {}", peer);
            let mut socket = TcpStream::connect(*peer).ok().expect("failed to open connection to peer");
            forward_join.serialize(&mut socket, &self.config.local_addr);
        }

        self.add_to_active_view(sender);
        self.send_join_ack(sender);
    }

    /// Pass in the read lock for the active_view as all callers have probably already aquired the read lock anyways, so eliminate the need for 
    /// obtaining another.
    fn add_to_active_view(&self, peer: &SocketAddr) {
        // add to the active list if node is not already in it
        println!("in add_to_active_view");
        if !HyParViewContext::contains(&*self.active_view.read().unwrap(), peer) {
            println!("{}, not in active_view, so adding it; active_View_size = {}", peer, self.config.active_view_size);
            self.active_view.write().unwrap().push(*peer);

            while self.active_view.read().unwrap().len() > self.config.active_view_size {
                println!("remove random node from active_list");
                let removed = self.active_view.write().unwrap().remove(0);
                self.add_to_passive_view(&removed);

                let mut socket = TcpStream::connect(removed).ok().expect("failed to open connection to peer (to disconnect)");
                let discon = Disconnect::new();
                discon.serialize(&mut socket, &self.config.local_addr);
            }
        }

        // check if the node is in the passive view, and remove it
        // also, we should never get into the situation where a node is in both the active and passive lists
        // but having this as a separate check helps prevent against it (argh, fucking data races)
        let contains = HyParViewContext::index_of(&*self.passive_view.read().unwrap(), peer);
        if contains.is_some() {
            self.passive_view.write().unwrap().remove(contains.unwrap());
        }
        println!("end of add_to_active_view");
    }

    fn add_to_passive_view(&self, peer: &SocketAddr) {
        self.passive_view.write().unwrap().push(*peer);
        let contains = HyParViewContext::index_of(&*self.passive_view.read().unwrap(), peer);
        if contains.is_none() {
            while self.passive_view.read().unwrap().len() > self.config.passive_view_size -1 {
                self.passive_view.write().unwrap().remove(0);
            }
        }
        self.passive_view.write().unwrap().push(*peer);
    }

    fn send_join_ack(&self, peer: &SocketAddr) {
        // send an 'ack' message back to the sender - currently in leiu of maintaining an open, mutable tcp connection (but I like this anyways :) )
        println!("sending join ack to {}", peer);
        let mut conn = TcpStream::connect(*peer).ok().expect("could not connect to node that wants to JOIN");
        let ack = JoinAck::new();
        ack.serialize(&mut conn, &self.config.local_addr);
    }

    //TODO: there *must* be a better way to do this rather than reusing a java-ism :(
    // also, could sort the nodes and do a binary search, but looks like the built-in vec.search() requires 2 * n space and n log n time - yuck!
    fn index_of(nodes: &Vec<SocketAddr>, target: &SocketAddr) -> Option<uint> {
        let mut idx = -1;
        for sock in nodes.iter() {
            idx += 1;
            if sock.eq(target) {
                return Some(idx);
            }
        }
        None
    }

    fn contains(nodes: &Vec<SocketAddr>, target: &SocketAddr) -> bool {
        let idx = HyParViewContext::index_of(nodes, target);
        idx.is_some()
    }

    fn handle_forward_join(&self, msg: &ForwardJoin, sender: &SocketAddr) {
        println!("in handle_forward_join from {}", sender);
        if self.active_view.read().unwrap().len() <= 1 || msg.ttl == 0 {
            self.add_to_active_view(&msg.originator);
            self.send_join_ack(&msg.originator);
            return;
        } 

        if msg.ttl == msg.prwl {
            self.add_to_passive_view(&msg.originator);
        }

        let ttl = msg.ttl - 1;
        let forward_join = ForwardJoin::new(&msg.originator, msg.arwl, msg.prwl, ttl);
        let active_view = self.active_view.read().unwrap();
        loop {
            let rand: uint = rand::random();
            let idx = rand % active_view.len();
            let peer = active_view[idx];
            if !peer.eq(sender) {
                let mut socket = TcpStream::connect(peer).ok().expect("failed to open connection to peer (to forward join)");
                forward_join.serialize(&mut socket, &self.config.local_addr);
                break;
            }
        }
    }

    fn handle_join_ack(&self, sender: &SocketAddr) {
        self.add_to_active_view(sender);
    }

    fn handle_disconnect(&self, sender: &SocketAddr) {
        // remove from active list, if currently in it
        let contains = HyParViewContext::index_of(&*self.active_view.read().unwrap(), sender);
        if contains.is_some() {
            self.active_view.write().unwrap().remove(contains.unwrap());
        }

        // add to the passive list, if not in it (either due to data race or programming bug)
        let contains = HyParViewContext::index_of(&*self.passive_view.read().unwrap(), sender);
        if contains.is_none() {
            while self.passive_view.read().unwrap().len() >= self.config.passive_view_size - 1 {
                let rand: uint = rand::random();
                let idx = rand % self.passive_view.read().unwrap().len();
                self.passive_view.write().unwrap().remove(idx);
            }
            self.passive_view.write().unwrap().push(*sender);
        }

        self.send_neighbor_request(None);
    }

    fn send_neighbor_request(&self, last_addr: Option<&SocketAddr>) {
        // this is a bit optimistic in that after we successfully send the NIEGHBORBOR msg, we expect to get some response (async, of course).
        // we should keep around some timeout reference so we can try another peer (and possibly kick out the one that timed out), but that's for the future.
        let mut idx = match last_addr {
            None => 0,
            Some(addr) => match HyParViewContext::index_of(&*self.passive_view.read().unwrap(), addr) {
                Some(i) => i,
                None => 0,
            }
        };

        // bail out if we've already cycled through all the possible peers -- unless the active view is now empty
        if idx >= self.passive_view.read().unwrap().len() {
            if self.active_view.read().unwrap().len() == 0 {
                idx = 0
            } else {
                return;
            }
        } 

        for i in range(idx, self.passive_view.read().unwrap().len()) {
            let priority = match self.active_view.read().unwrap().len() {
                0 => Priority::High,
                _ => Priority::Low,
            };
            let neighbor = NeighborRequest::new(priority);

            match TcpStream::connect_timeout(self.passive_view.read().unwrap()[i], Duration::seconds(4)) {
                Ok(ref mut socket) => {
                    neighbor.serialize(&mut* socket, &self.config.local_addr);
                    break;
                },
                Err(_) => {
                    self.passive_view.write().unwrap().remove(i);
                },
            }
            idx += 1;
        }
    }

    fn handle_neighbor_request(&self, msg: &NeighborRequest, sender: &SocketAddr) {
        let mut socket = TcpStream::connect(*sender).ok().expect("failed to open connection to peer (to forward join)");
        if Priority::Low.eq(&msg.priority) && self.active_view.read().unwrap().len() == self.config.active_view_size {
            let resp = NeighborResponse::new(Result::Reject);
            resp.serialize(&mut socket, &self.config.local_addr);
            return;
        }

        self.add_to_active_view(sender);
        let resp = NeighborResponse::new(Result::Accept);
        resp.serialize(&mut socket, &self.config.local_addr);
    }

    fn handle_neighbor_response(&self, msg: &NeighborResponse, sender: &SocketAddr) {
        match msg.result {
            Result::Accept => self.add_to_active_view(sender),
            Result::Reject => self.send_neighbor_request(Some(sender)),
        };
    }

    fn handle_shuffle(&self, msg: Shuffle, sender: &SocketAddr) {
        // first, determine if this node should handle the request or pass it on down
        if msg.ttl > 0 && self.active_view.read().unwrap().len() > 1 {
            let &mut addr = sender;
            while addr.eq(sender) {
                addr = HyParViewContext::select_random(&*self.active_view.read().unwrap());
            }
            //TODO better error handling around opening the connection
            let mut socket = TcpStream::connect(addr).ok().expect("failed to open connection to peer (to forward the shuffle)");
            let shuffle = Shuffle::new(msg.originator, msg.nodes, msg.ttl - 1);
            shuffle.serialize(&mut socket, &self.config.local_addr);
            return;
        }

        let nodes = self.build_shuffle_list(&*self.active_view.read().unwrap(), &msg.originator);
        let empty_vec = Vec::new();
        self.apply_shuffle(&msg.nodes, &empty_vec);

        let shuffle_reply = ShuffleReply::new(msg.nodes, nodes);
        let mut socket = TcpStream::connect(msg.originator).ok().expect("failed to open connection to peer");
        shuffle_reply.serialize(&mut socket, &self.config.local_addr);
    }

    fn apply_shuffle(&self, nodes: &Vec<SocketAddr>, filter: &Vec<SocketAddr>) {
        let mut filter_idx = 0;
        let filter_len = filter.len();

        for node in nodes.iter() {
            // check to see if node is in active_view or passive_view - skip node if it is
            if HyParViewContext::contains(&*self.active_view.read().unwrap(), node) || HyParViewContext::contains(&*self.passive_view.read().unwrap(), node) {
                continue;
            }

            // if passive_view at limit, remove one of the nodes as ref'd in the filter array (or a random node is filter is exhausted)
            while self.passive_view.read().unwrap().len() >= self.config.passive_view_size - 1 {
                if filter_len > 0 && filter_idx < filter_len {
                    let cur = filter[filter_idx];
                    filter_idx += 1;
                    let idx = HyParViewContext::index_of(&*self.passive_view.read().unwrap(), &cur);
                    if idx.is_some() {
                        self.passive_view.write().unwrap().remove(idx.unwrap());
                    } else {
                        continue;
                    }
                } else {
                    let rand: uint = rand::random();
                    let idx = rand % self.passive_view.write().unwrap().len();
                    self.passive_view.write().unwrap().remove(idx);
                }
            }
            self.passive_view.write().unwrap().push(*node);
        }
    }

    fn handle_shuffle_reply(&self, msg: &ShuffleReply, sender: &SocketAddr) {
        self.apply_shuffle(&msg.nodes, &msg.sent_nodes);
    }
}

pub fn start_service(config: Arc<Config>, rx: Receiver<HyParViewMessage>) {
    println!("starting up hyparview");
    let hpv = HyParViewContext::new(config);
    let ctx = Arc::new(hpv);

    let ctx_clone = ctx.clone();
    Thread::spawn(move || {
        // some initial delay
        sleep(Duration::seconds(10));
        ctx_clone.next_round();
    }).detach();

    let ctx_clone = ctx.clone();
    Thread::spawn(move ||  {
        ctx_clone.listen(rx);
    }).detach();

    ctx.join();
}
