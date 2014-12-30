extern crate config;

use config::Config;
use messages::{HyParViewMessage,Disconnect,ForwardJoin,Join,JoinAck,NeighborRequest,NeighborResponse,Priority};
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

    // NOTE: not sure I'm really doing the best thing here using RWLock, but it's allowing me to mutate the Vec, so I think I"m on the right path there.
    // however, I don't think i can retain a mutable reference to the open, outbound tcp connection (with the socket addr). thus, am punting on it for now..
    active_view: RWLock<Vec<SocketAddr>>,
    passive_view: RWLock<Vec<SocketAddr>>
}
impl HyParViewContext {
    fn new(config: Arc<Config>) -> HyParViewContext {
        HyParViewContext { 
            config: config,
            active_view: RWLock::new(Vec::new()),
            passive_view: RWLock::new(Vec::new()),
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

    /// next SHUFFLE round, that is!
    pub fn next_round(&self) {
        println!("in next round");
        let mut timer = Timer::new().unwrap();
        let periodic = timer.periodic(Duration::seconds(10));
        loop {
            periodic.recv();
            if self.active_view.read().len() == 0 {
                self.join();
                continue;
            }

            //TODO: perform the shuffle operation
        }
    }

    pub fn listen(&self, rx: Receiver<HyParViewMessage>) {
        println!("starting listen()");

        loop {
            println!("revc'd and processing hyparview msg");
            // TODO: try_recv() does *not* block, and might be nice for a gentle shutdown of the listener
            match rx.recv() {
                HyParViewMessage::JoinMessage(msg,addr) => self.handle_join(&addr),
                HyParViewMessage::ForwardJoinMessage(msg,addr) => self.handle_forward_join(&msg, &addr),
                HyParViewMessage::JoinAckMessage(msg,addr) => self.handle_join_ack(&addr),
                HyParViewMessage::DisconnectMessage(msg,addr) => self.handle_disconnect(&addr),
                HyParViewMessage::NeighborRequestMessage(msg,addr) => self.handle_neighbor_request(&msg, &addr),
                HyParViewMessage::NeighborResponseMessage(msg,addr) => self.handle_neighbor_response(&msg, &addr),
            }
        }
    }

    /// a contact_node (or really any node, for that matter) receives a JOIN request from a node that wants to join the cluster.
    /// the requesting could be a completely new node, or it could be a node that bounced or possibly it resent the JOIN request
    /// due to timeout (because it didn't receive a response).
    fn handle_join(&self, sender: &SocketAddr) {
        self.add_to_active_view(sender);
        self.send_join_ack(sender);

        let active_view = self.active_view.read();
        let forward_join = ForwardJoin::new(sender, self.config.active_random_walk_length, self.config.passive_random_walk_length, self.config.active_random_walk_length);
        for peer in active_view.iter() {
            let mut socket = TcpStream::connect(*peer).ok().expect("failed to open connection to peer");
            forward_join.serialize(&mut socket);
        }
    }

    fn add_to_active_view(&self, peer: &SocketAddr) {
        // add to the active list if node is not already in it
        let active_view = self.active_view.read();
        let contains = HyParViewContext::index_of(&*active_view, peer);
        if contains.is_none() {
            let mut active_view_lock =  self.active_view.write();
            while active_view.len() >= self.config.active_view_size - 1 {
                let rand: uint = rand::random();
                let idx = rand % active_view.len();

                let removed = active_view_lock.remove(idx).expect("should have the element we just tried to remove - else, we lost a data race");
                // send disconnect message
                let mut socket = TcpStream::connect(removed).ok().expect("failed to open connection to peer (to disconnect)");
                let discon = Disconnect::new();
                discon.serialize(&mut socket);
            }

            active_view_lock.push(*peer);
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
        let contains = HyParViewContext::index_of(&*passive_view, peer);
        if contains.is_none() {
            while passive_view.len() >= self.config.passive_view_size - 1 {
                // could try something fancy like removing a random entry, but screw it, let's go simple!
                passive_view_write.remove(0);
            }
        }
        passive_view_write.push(*peer);
    }

    fn send_join_ack(&self, peer: &SocketAddr) {
        // send an 'ack' message back to the sender - currently in leiu of maintaining an open, mutable tcp connection (but I like this anyways :) )
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

    fn handle_forward_join(&self, msg: &ForwardJoin, sender: &SocketAddr) {
        let active_view = self.active_view.read();
        if active_view.len() <= 1 || msg.ttl == 0 {
            self.add_to_active_view(&msg.originator);
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
        self.add_to_active_view(sender);
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

        // this is a bit optimistic in that after we successfully send the NIEGHBORBOR msg, we expect to get some response (async, of course).
        // we should keep around some timeout reference so we can try another peer (and possibly kick out the one that timed out), but that's for the future.
        loop {
            let priority = match active_view.len() {
                0 => Priority::High,
                _ => Priority::Low,
            };
            let neighbor = NeighborRequest::new(priority);
            let mut idx = 0;
            for peer in passive_view.iter() {
                match TcpStream::connect_timeout(*peer, Duration::seconds(4)) {
                    Ok(ref mut socket) => {
                        neighbor.serialize(&mut* socket);
                        break;
                    },
                    Err(e) => {
                        self.passive_view.write().remove(idx);
                    },
                }
                idx += 1;
            }
        }
    }

    fn handle_neighbor_request(&self, msg: &NeighborRequest, sender: &SocketAddr) {
        // if msg.priority.High 

        // if self.active_list.read().len() < self.config.active_view_size
    }

    fn handle_neighbor_response(&self, msg: &NeighborResponse, sender: &SocketAddr) {

    }
}

pub fn start_service(config: Arc<Config>) -> Sender<HyParViewMessage> {
    println!("starting up hyparview");
    let hpv = HyParViewContext::new(config);
    let ctx = Arc::new(hpv);

    let ctx_clone = ctx.clone();
    Thread::spawn(move || {
        // some initial delay
        sleep(Duration::seconds(10));
        ctx_clone.next_round();
    }).detach();

    // setup the task that listens to incoming messages
    let (tx, rx) : (Sender<HyParViewMessage>, Receiver<HyParViewMessage>) = channel();
    let ctx_clone = ctx.clone();
    Thread::spawn(move ||  {
        ctx_clone.listen(rx);
    }).detach();

    ctx.join();
    tx
}
