use messages::{HyParViewMessage,Join,ForwardJoin};
use std::io::Timer;
use std::io::net::ip::SocketAddr;
use std::io::net::tcp::TcpStream;
use std::rand;
use std::sync::{Arc};
use std::time::Duration;
use std::thread::Thread;
use std::vec::Vec;

pub mod messages;

pub struct HyParViewContext {
    local: SocketAddr,
    contact_nodes: Vec<SocketAddr>,
    active_view: Vec<SocketAddr>,
    passive_view: Vec<SocketAddr>
}
impl HyParViewContext {
    fn new(config: &Config) -> HyParViewContext {
//    fn new(local: SocketAddr, contact_nodes: Vec<SocketAddr>) -> HyParViewContext {
        HyParViewContext { 
            local: local,
            contact_nodes: contact_nodes,
            active_view: Vec::new(),
            passive_view: Vec::new(),
        }
    }

    /// call a random contact_node and send a 'JOIN' message
    pub fn join(&self) {
        println!("in join()");
       // TODO: add callback to ensure we got a response (Neighbor) msg from some peer

         if self.contact_nodes.len() == 0 || self.contact_nodes.len() == 1 && self.contact_nodes[0].eq(&self.local) {
            println!("no unique contact node addresses available");
            return;
        }

        let rand: uint = rand::random();
        let idx = rand % self.contact_nodes.len();
        let node = self.contact_nodes[idx];
        let mut socket = TcpStream::connect(node);
        let msg = Join::new(&self.local);
        msg.serialize(&mut socket);
    }

    pub fn next_round(&self) {
        println!("in next round");
        let mut timer = Timer::new().unwrap();
        let periodic = timer.periodic(Duration::seconds(1));
        loop {
//            println!("** performing next round");
            periodic.recv();
            // TODO: do HyParView stuffs here!
        }
    }

    pub fn listen(&self, rx: Receiver<HyParViewMessage>) {
        println!("starting listen()");

        loop {
            println!("revc'd and processing hyparview msg");
            // TODO: try_recv() does *not* block, and might be nice for a gentle shutdown of the listener
            match rx.recv() {
                HyParViewMessage::JoinMessage(msg) => self.handle_join(&msg),
                HyParViewMessage::ForwardJoinMessage(msg) => self.handle_forward_join(&msg),
            }
        }
    }

    /// a contact_node (or really any node, for that matter) receives a JOIN request from a node that wants to join the cluster.
    /// the requesting could be a completely new node, or it could be a node that bounced or possibly it resent the JOIN request
    /// due to timeout (because it didn't receive a response).
    fn handle_join(&self, msg: &Join) {
        // add to my active list 
        if HyParViewContext::index_of(self.active_view, &msg.sender) < 0 {
            
        } else {
            let idx = HyParViewContext::index_of(self.passive_view, &msg.sender);
            if idx >= 0 {
                self.passive_view.remove();
            }
        }
        self.active_view.push(msg.sender);
        
        // construct ForwardJoin message 
        // broadcast to everyone in my active list (except the new node, of course)
    }

    //TODO: there *must* be a better way to do this rather than reusing a java-ism :(
    // also, could sort the nodes and do a binary search, but looks like the built-in vec.search() requires 2 * n space and n log n time - yuck!
    fn index_of(nodes: &Vec, target: &SocketAddr) -> u8 {
        let mut idx = -1;
        for sock in nodes.iter() {
            idx += 1;
            if sock.eq(target) {
                return idx;
            }
        }
        -1
    }

    fn handle_forward_join(&self, msg: &ForwardJoin) {
        
    }
}

pub fn start_service(local: SocketAddr, contact_nodes: Vec<SocketAddr>) -> Sender<HyParViewMessage> {
    println!("starting up hyparview");
    let hpv = HyParViewContext::new(local, contact_nodes);
    let ctx = Arc::new(hpv);

    let ctx_clone = ctx.clone();
    Thread::spawn(move || {
        //TODO: add some initial delay
        
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



