extern crate config;

use config::Config;
use messages::{HyParViewMessage,Disconnect,ForwardJoin,Join,JoinAck,NeighborRequest,NeighborResponse,Priority,Result,Shuffle,ShuffleReply};
use std::io::Timer;
use std::io::net::ip::SocketAddr;
use std::io::net::tcp::TcpStream;
use std::io::timer::sleep;
use std::rand;
use std::sync::{Arc,RWLock};
use std::time::Duration;
use std::thread::Thread;
use std::vec::Vec;

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
        let msg = Join::new();
        let mut socket = TcpStream::connect(node).ok().expect("failed to open connection to peer");
        msg.serialize(&mut socket);
    }

    pub fn next_round(&self) {
        let mut timer = Timer::new().unwrap();
        let periodic = timer.periodic(Duration::seconds(10));
        loop {
            periodic.recv();
            println!("start of next shuffle round");
            match self.active_view.read().len() {
                0 => self.join(),
                _ => self.do_shuffle(),
            }
        }
    }

    fn do_shuffle(&self) {
        let active_view = self.active_view.read();
        let target_addr = HyParViewContext::select_random(&*active_view);
        let nodes = self.build_shuffle_list(&*active_view, &target_addr);
        let shuffle = Shuffle::new(self.config.local_addr, nodes, self.config.shuffle_walk_length);
        let mut socket = TcpStream::connect(target_addr).ok().expect("failed to open connection to peer");
        shuffle.serialize(&mut socket);

        // additionally, (TODO: probablistically) choose to send to a contact_node
        let addr = HyParViewContext::select_random(&self.config.contact_nodes); 
        let mut socket = TcpStream::connect(addr).ok().expect("failed to open connection to peer");
        shuffle.serialize(&mut socket);
    }

    fn build_shuffle_list(&self, active_view: &Vec<SocketAddr>, target_addr: &SocketAddr) -> Vec<SocketAddr> {
        let active_cnt = self.config.shuffle_active_view_count.to_uint().unwrap();
        let mut nodes = Vec::with_capacity(1u + active_cnt + self.config.shuffle_passive_view_count.to_uint().unwrap());
        nodes.push(self.config.local_addr);
        nodes.push(*target_addr);

        HyParViewContext::select_multiple_random(&*active_view, &mut nodes, 1u + active_cnt);
        let passive_view = self.passive_view.read();
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
        println!("starting listen()");

        loop {
            // TODO: try_recv() does *not* block, and might be nice for a gentle shutdown of the listener
            match rx.recv() {
                HyParViewMessage::JoinMessage(_,addr) => self.handle_join(&addr),
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
    fn handle_join(&self, sender: &SocketAddr) {
        let active_view = self.active_view.read();
        let forward_join = ForwardJoin::new(sender, self.config.active_random_walk_length, self.config.passive_random_walk_length, self.config.active_random_walk_length);
        for peer in active_view.iter() {
            let mut socket = TcpStream::connect(*peer).ok().expect("failed to open connection to peer");
            forward_join.serialize(&mut socket);
        }

        self.add_to_active_view(sender, &*active_view);
        self.send_join_ack(sender);
    }

    /// Pass in the read lock for the active_view as all callers have probably already aquired the read lock anyways, so eliminate the need for 
    /// obtaining another.
    fn add_to_active_view(&self, peer: &SocketAddr, active_view: &Vec<SocketAddr>) {
        // add to the active list if node is not already in it
        let contains = HyParViewContext::index_of(&*active_view, peer);
        if contains.is_none() {
            let mut active_view_write =  self.active_view.write();
            active_view_write.push(*peer);
            while active_view.len() > self.config.active_view_size {
                let removed = active_view_write.remove(0).expect("should have the element we just tried to remove - else, we lost a data race");
                self.add_to_passive_view(&removed);

                let mut socket = TcpStream::connect(removed).ok().expect("failed to open connection to peer (to disconnect)");
                let discon = Disconnect::new();
                discon.serialize(&mut socket);
            }
        }

        // check if the node is in the passive view, and remove it
        // also, we should never get into the situation where a node is in both the active and passive lists
        // but having this as a separate check helps prevent against it (argh, fucking data races)
        let contains = HyParViewContext::index_of(&*self.passive_view.read(), peer);
        if contains.is_some() {
            self.passive_view.write().remove(contains.unwrap());
        }
    }

    fn add_to_passive_view(&self, peer: &SocketAddr) {
        let passive_view = self.passive_view.read();
        let mut passive_view_write = self.passive_view.write();
        passive_view_write.push(*peer);
        let contains = HyParViewContext::index_of(&*passive_view, peer);
        if contains.is_none() {
            while passive_view.len() > self.config.passive_view_size {
                passive_view_write.remove(0);
            }
        }
    }

    fn send_join_ack(&self, peer: &SocketAddr) {
        // send an 'ack' message back to the sender - currently in leiu of maintaining an open, mutable tcp connection (but I like this anyways :) )
        println!("sending join ack");
        let mut conn = TcpStream::connect(*peer).ok().expect("could not connect to node that wants to JOIN");
        let ack = JoinAck::new();
        ack.serialize(&mut conn);
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
        let active_view = self.active_view.read();
        if active_view.len() <= 1 || msg.ttl == 0 {
            self.add_to_active_view(&msg.originator, &*active_view);
            self.send_join_ack(&msg.originator);
            return;
        } 

        if msg.ttl == msg.prwl {
            self.add_to_passive_view(&msg.originator);
            return;
        }

        let ttl = msg.ttl - 1;
        let forward_join = ForwardJoin::new(&msg.originator, msg.arwl, msg.prwl, ttl);
        loop {
            let rand: uint = rand::random();
            let idx = rand % active_view.len();
            let peer = active_view[idx];
            if !peer.eq(sender) {
                let mut socket = TcpStream::connect(peer).ok().expect("failed to open connection to peer (to forward join)");
                forward_join.serialize(&mut socket);
                break;
            }
        }
    }

    fn handle_join_ack(&self, sender: &SocketAddr) {
        let active_view = self.active_view.read();
        self.add_to_active_view(sender, &*active_view);
    }

    fn handle_disconnect(&self, sender: &SocketAddr) {
        // remove from active list, if currently in it
        let active_view = self.active_view.read();
        let contains = HyParViewContext::index_of(&*active_view, sender);
        if contains.is_some() {
            self.active_view.write().remove(contains.unwrap());
        }

        // add to the passive list, if not in it (either due to data race or programming bug)
        let passive_view = self.passive_view.read();
        let contains = HyParViewContext::index_of(&*passive_view, sender);
        if contains.is_none() {
            while passive_view.len() >= self.config.passive_view_size - 1 {
                let rand: uint = rand::random();
                let idx = rand % passive_view.len();
                self.passive_view.write().remove(idx);
            }
            self.passive_view.write().push(*sender);
        }

        self.send_neighbor_request(&*active_view, &*passive_view, None);
    }

    fn send_neighbor_request(&self, active_view: &Vec<SocketAddr>, passive_view: &Vec<SocketAddr>, last_addr: Option<&SocketAddr>) {
        // this is a bit optimistic in that after we successfully send the NIEGHBORBOR msg, we expect to get some response (async, of course).
        // we should keep around some timeout reference so we can try another peer (and possibly kick out the one that timed out), but that's for the future.
        let mut idx = match last_addr {
            None => 0,
            Some(addr) => match HyParViewContext::index_of(passive_view, addr) {
                Some(i) => i,
                None => 0,
            }
        };

        // bail out if we've already cycled through all the possible peers -- unless the active view is now empty
        if idx >= passive_view.len() {
            if active_view.len() == 0 {
                idx = 0
            } else {
                return;
            }
        } 

        for i in range(idx, passive_view.len()) {
            let priority = match active_view.len() {
                0 => Priority::High,
                _ => Priority::Low,
            };
            let neighbor = NeighborRequest::new(priority);

            match TcpStream::connect_timeout(passive_view[i], Duration::seconds(4)) {
                Ok(ref mut socket) => {
                    neighbor.serialize(&mut* socket);
                    break;
                },
                Err(_) => {
                    self.passive_view.write().remove(i);
                },
            }
            idx += 1;
        }
    }

    fn handle_neighbor_request(&self, msg: &NeighborRequest, sender: &SocketAddr) {
        let active_view = self.active_view.read();
        let mut socket = TcpStream::connect(*sender).ok().expect("failed to open connection to peer (to forward join)");
        if Priority::Low.eq(&msg.priority) && active_view.len() == self.config.active_view_size {
            let resp = NeighborResponse::new(Result::Reject);
            resp.serialize(&mut socket);
            return;
        }

        self.add_to_active_view(sender, &*active_view);
        
        let resp = NeighborResponse::new(Result::Accept);
        resp.serialize(&mut socket);
    }

    fn handle_neighbor_response(&self, msg: &NeighborResponse, sender: &SocketAddr) {
        let active_view = self.active_view.read();
        match msg.result {
            Result::Accept => self.add_to_active_view(sender, &*active_view),
            Result::Reject => self.send_neighbor_request(&*active_view, &*self.passive_view.read(), Some(sender)),
        };
    }

    fn handle_shuffle(&self, msg: Shuffle, sender: &SocketAddr) {
        // first, determine if this node should handle the request or pass it on down
        let active_view = self.active_view.read();
        if msg.ttl > 0 && active_view.len() > 1 {
            let &mut addr = sender;
            while addr.eq(sender) {
                addr = HyParViewContext::select_random(&*active_view);
            }
            //TODO better error handling around opening the connection
            let mut socket = TcpStream::connect(addr).ok().expect("failed to open connection to peer (to forward the shuffle)");
            let shuffle = Shuffle::new(msg.originator, msg.nodes, msg.ttl - 1);
            shuffle.serialize(&mut socket);
            return;
        }

        let nodes = self.build_shuffle_list(&*active_view, &msg.originator);
        let empty_vec = Vec::new();
        self.apply_shuffle(&msg.nodes, &empty_vec);

        let shuffle_reply = ShuffleReply::new(msg.nodes, nodes);
        let mut socket = TcpStream::connect(msg.originator).ok().expect("failed to open connection to peer");
        shuffle_reply.serialize(&mut socket);
    }

    fn apply_shuffle(&self, nodes: &Vec<SocketAddr>, filter: &Vec<SocketAddr>) {
        let mut filter_idx = 0;
        let active_view = self.active_view.read();
        let passive_view = self.passive_view.read();
        let mut passive_view_write = self.passive_view.write();
        let passive_cnt = self.config.passive_view_size;
        let filter_len = filter.len();

        for node in nodes.iter() {
            // check to see if node is in active_view or passive_view - skip node if it is
            if HyParViewContext::contains(&*active_view, node) || HyParViewContext::contains(&*passive_view, node) {
                continue;
            }

            // if passive_view at limit, remove one of the nodes as ref'd in the filter array (or a random node is filter is exhausted)
            while passive_view.len() >= passive_cnt {
                if filter_len > 0 && filter_idx < filter_len {
                    let cur = filter[filter_idx];
                    filter_idx += 1;
                    let idx = HyParViewContext::index_of(&*passive_view, &cur);
                    if idx.is_some() {
                        passive_view_write.remove(idx.unwrap());
                    } else {
                        continue;
                    }
                } else {
                    let rand: uint = rand::random();
                    let idx = rand % passive_view.len();
                    passive_view_write.remove(idx);
                }
            }
            passive_view_write.push(*node);
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
