use std::io::Timer;
use std::io::net::ip::SocketAddr;
use std::vec::Vec;
use std::sync::{Arc};
use std::time::Duration;
use std::thread::Thread;

struct HyParViewContext {
    local: SocketAddr,
    seeds: Vec<SocketAddr>,
    active_view: Vec<SocketAddr>,
    passive_view: Vec<SocketAddr>
}
impl HyParViewContext {
    fn new(local: SocketAddr, seeds: Vec<SocketAddr>) -> HyParViewContext {
        HyParViewContext { 
            local: local,
            seeds: seeds,
            active_view: Vec::new(),
            passive_view: Vec::new(),
        }
    }

    pub fn listen(&self, rx: Receiver<int>) {
        println!("starting listen()");

        // need to loop here!
        let incoming = rx.recv();
        println!("revc'd int: {}", incoming);
    }

    pub fn next_round(&self) {
        println!("in next round");
        let mut timer = Timer::new().unwrap();
        let periodic = timer.periodic(Duration::seconds(1));
        loop {
            periodic.recv();
            // TODO: do HyParView stuffs here!
        }
    }
}

pub fn start_service(local: SocketAddr, seeds: Vec<SocketAddr>) -> Sender<int> {
    println!("starting up hyparview");
    let hpv = HyParViewContext::new(local, seeds);
    let ctx = Arc::new(hpv);

    let ctx_clone = ctx.clone();
    Thread::spawn(move || {
        ctx_clone.next_round();
    });

    // setup the task that listens to incoming messages
    let (tx, rx) : (Sender<int>, Receiver<int>) = channel();
    let ctx_clone = ctx.clone();
    Thread::spawn(move ||  {
        ctx_clone.listen(rx);
    });

    tx
}



