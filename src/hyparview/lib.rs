use std::io::Timer;
use std::io::net::ip::SocketAddr;
use std::vec::Vec;
use std::sync::{Arc};
use std::time::Duration;
use std::thread::Thread;

struct HyParViewContext {
    local: SocketAddr,
    contact_nodes: Vec<SocketAddr>,
    active_view: Vec<SocketAddr>,
    passive_view: Vec<SocketAddr>
}
impl HyParViewContext {
    fn new(local: SocketAddr, contact_nodes: Vec<SocketAddr>) -> HyParViewContext {
        HyParViewContext { 
            local: local,
            contact_nodes: contact_nodes,
            active_view: Vec::new(),
            passive_view: Vec::new(),
        }
    }

    pub fn listen(&self, rx: Receiver<int>) {
        println!("starting listen()");

        loop {
            // TODO: try_recv() does *not* block, and might be nice for a gentle shutdown of the listener
            let incoming = rx.recv(); 
            println!("revc'd int: {}", incoming);
        }
    }

    pub fn next_round(&self) {
        println!("in next round");
        let mut timer = Timer::new().unwrap();
        let periodic = timer.periodic(Duration::seconds(1));
        loop {
            println!("** performing next round");
            periodic.recv();
            // TODO: do HyParView stuffs here!
        }
    }
}

pub fn start_service(local: SocketAddr, contact_nodes: Vec<SocketAddr>) -> Sender<int> {
    println!("starting up hyparview");
    let hpv = HyParViewContext::new(local, contact_nodes);
    let ctx = Arc::new(hpv);

    let ctx_clone = ctx.clone();
    Thread::spawn(move || {
        ctx_clone.next_round();
    }).detach();

    // setup the task that listens to incoming messages
    let (tx, rx) : (Sender<int>, Receiver<int>) = channel();
    let ctx_clone = ctx.clone();
    Thread::spawn(move ||  {
        ctx_clone.listen(rx);
    }).detach();

    tx
}



