#![feature(core)]
#![feature(io)]
#![feature(std_misc)]

#[macro_use] extern crate log;
extern crate polaris;

use polaris::config::Config;
use polaris::hyparview::{HyParViewContext};
use polaris::hyparview::messages::{HyParViewMessage};
use polaris::shipper::{Serializable,Shipper};
use std::collections::HashMap;
use std::old_io::{MemReader,BufferedWriter,IoResult};
use std::old_io::net::ip::{SocketAddr};
use std::old_io::timer::Timer;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel,Sender,Receiver};
use std::time::Duration;

struct SimpleShipper {
    // a cruddy way around having mutable state in a struct -- yes, i need to learn more....
    pub invoked: AtomicBool,
}
impl SimpleShipper {
    fn new() -> SimpleShipper {
        SimpleShipper { invoked: AtomicBool::new(false) }
    }
}
impl Shipper for SimpleShipper {
    #[allow(unused_variables)]
    fn ship(&self, msg: &Serializable, dest: &SocketAddr) -> bool {
        self.invoked.store(true, Ordering::Relaxed);
        true
    }
}

struct InMemoryShipper {
    local_addr: SocketAddr,
    sender: Sender<(HyParViewMessage, SocketAddr)>,
}
impl InMemoryShipper {
    pub fn new(addr: SocketAddr, sender: Sender<(HyParViewMessage, SocketAddr)>) -> InMemoryShipper {
        InMemoryShipper { local_addr: addr, sender: sender }
    }
}
impl Shipper for InMemoryShipper {
    fn ship(&self, msg: &Serializable, dest: &SocketAddr) -> bool {
        match convert(msg, &self.local_addr) {
            Ok(hpv_msg) => {
                println!("converted msg {:?}, from {} to {}", hpv_msg, self.local_addr, dest);
                match self.sender.send((hpv_msg, *dest)) {
                    Ok(_) => assert!(true),
                    Err(e) => assert!(false, format!("failed to convert outbound message to an event: {}", e)),
                }
            },
            Err(e) => assert!(false, format!("failed to convert serializable to a HyParViewMessage: {}", e)),
        }
        true
    }
}

#[allow(unused_variables)]
fn convert(msg: &Serializable, src_addr: &SocketAddr) -> IoResult<HyParViewMessage> {
    let mut writer = BufferedWriter::new(Vec::with_capacity(256));
    let result = msg.serialize(&mut writer, src_addr);
    let mut reader = MemReader::new(writer.into_inner());
    polaris::hyparview::messages::deserialize(&mut reader)
}

fn build_node(socket: SocketAddr, seeds: &Vec<SocketAddr>) -> HyParViewContext {
    // clone the seeds list as we can't share the ownership. this is OK since we're only in a test, a each node would
    // be a separate process, at a minimum, if not on a different box
    let mut seeds_clone: Vec<SocketAddr> = Vec::with_capacity(seeds.len());
    for addr in seeds.iter() {
        seeds_clone.push(addr.clone());
    }

    let config = Config { 
        local_addr: socket, contact_nodes: seeds_clone, active_random_walk_length: 3, passive_random_walk_length: 2,
        active_view_size: 3, passive_view_size: 7, shuffle_period_seconds: 15, shuffle_active_view_count: 2,
        shuffle_passive_view_count: 5, shuffle_walk_length: 3
    };

    HyParViewContext::new(Arc::new(config))
}

#[test]
fn one_node_join_seed_is_self() {
    let sock_addr: SocketAddr = ("127.0.0.1:9090").parse().unwrap();
    let mut seeds: Vec<SocketAddr> = Vec::with_capacity(1us);
    seeds.push(sock_addr);

    let mut shipper = SimpleShipper::new();
    let seed = build_node(sock_addr, &seeds);
    seed.join(&mut shipper);
    assert!(!shipper.invoked.load(Ordering::Relaxed));
}

#[test]
fn one_node_join_seed_is_other() {
    let sock_addr: SocketAddr = ("127.0.0.1:9090").parse().unwrap();
    let mut seeds: Vec<SocketAddr> = Vec::with_capacity(1us);
    let seed_sock_addr: SocketAddr = ("127.1.0.1:9090").parse().unwrap();
    seeds.push(seed_sock_addr);

    let mut shipper = SimpleShipper::new();
    let seed = build_node(sock_addr, &seeds);
    seed.join(&mut shipper);
    assert!(shipper.invoked.load(Ordering::Relaxed));
}

fn dispatch(nodes: &mut HashMap<SocketAddr, HyParViewContext>, sender: Sender<(HyParViewMessage, SocketAddr)>, receiver: Receiver<(HyParViewMessage, SocketAddr)>) {
    let mut timer = Timer::new().unwrap();
    let timeout = timer.oneshot(Duration::seconds(2));

    let mut cnt = 0;
    loop {
        select! {
            _ = timeout.recv() => break,
            r = receiver.recv() => {
                cnt += 1;
                match r {
                    Ok((msg, dest)) => {
                        match nodes.get_mut(&dest) {
                            Some(mut ctx) => {
                                println!("{})\tabout to dispatch a msg: {:?}, to {}", cnt, msg, dest);
                                let mut shipper = InMemoryShipper::new(dest, sender.clone());
                                ctx.receive_event(msg, &mut shipper);
                            },
                            None => assert!(false, format!("could not find dest addr in dispatch map: {}", dest)),
                        }
                    },
                    Err(e) => assert!(false, "failed to get message form channel: {:?}", e),
                }
            }
        }
    }
}

/// run the entire join sequence between two nodes, one is a seed, and the other not.
#[test]
fn join_two_nodes() {
    let mut nodes: HashMap<SocketAddr, HyParViewContext> = HashMap::new();

    let seed_addr: SocketAddr = ("127.0.0.1:9090").parse().unwrap();
    let mut seeds: Vec<SocketAddr> = Vec::with_capacity(1us);
    seeds.push(seed_addr);

    let seed = build_node(seed_addr, &seeds);
    nodes.insert(seed.config().local_addr, seed);

    let (sender, receiver) = channel::<(HyParViewMessage, SocketAddr)>();

    let peer_addr: SocketAddr = ("127.1.0.1:9090").parse().unwrap();
    let peer = build_node(peer_addr, &seeds);
    let mut starter_shipper = InMemoryShipper::new(peer_addr, sender.clone());
    peer.join(&mut starter_shipper);
    nodes.insert(peer.config().local_addr, peer);

    dispatch(&mut nodes, sender, receiver);

    let seed = nodes.get(&seed_addr).unwrap();
    assert_eq!(1, seed.active_view.len());
    assert!(seed.active_view[0].eq(&peer_addr));

    let peer = nodes.get(&peer_addr).unwrap();
    assert_eq!(1, peer.active_view.len());
    assert!(peer.active_view[0].eq(&seed_addr));
}

/// run the entire join sequence for a large number of nodes. As we'll have more nodes
/// than the active_view size, we'll exercise the forward_join functionality as well as
/// dropping nodes from the active_view / disconnecting.
#[test]
fn join_many_nodes() {
    let mut nodes: HashMap<SocketAddr, HyParViewContext> = HashMap::new();
    let (sender, receiver) = channel::<(HyParViewMessage, SocketAddr)>();

    let seed_cnt = 2us;
    let mut seeds: Vec<SocketAddr> = Vec::with_capacity(seed_cnt);
    for i in 1..(seed_cnt + 1) {
        let seed_addr: SocketAddr = (format!("127.0.0.{}:9090", i)).parse().unwrap();
        seeds.push(seed_addr);
        let seed = build_node(seed_addr, &seeds);
        nodes.insert(seed.config().local_addr, seed);
    }

    let cluster_size = 64us;
    assert!(seed_cnt < cluster_size);
    for i in (seed_cnt + 1)..(cluster_size + 1) {
        let peer_addr: SocketAddr = (format!("127.1.0.{}:9090", i)).parse().unwrap();
        let peer = build_node(peer_addr, &seeds);
        let mut starter_shipper = InMemoryShipper::new(peer_addr, sender.clone());
        peer.join(&mut starter_shipper);
        nodes.insert(peer.config().local_addr, peer);
    }

    dispatch(&mut nodes, sender, receiver);

    for addr in seeds.iter() {
        let seed = nodes.get(addr).unwrap();
//    assert_eq!(seed.config.active_view_size, seed.active_view.len());
        println!("resultant seed {} active view = {:?}", addr, seed.active_view);
    }

    assert_eq!(cluster_size, nodes.len());

    let check = true;
    // make sure all nodes have reciprocating entries in active_views
    for (addr, ctx) in nodes.iter() {
        println!("\n****** next node = addr: {}, \n\tctx.active view: {:?}, \n\tctx.passive view: {:?}", addr, ctx.active_view, ctx.passive_view);

        if !check {
            continue;
        }

        assert!(0 < ctx.active_view.len(), format!("node {} has empty active_view", addr));

        for peer in ctx.active_view.iter() {
            match nodes.get(peer) {
                Some(node) => match node.active_view.iter().position(|&x| x.eq(addr)) {
                    Some(_) => (),
                    None => assert!(false, format!("node {} does not have a reciprocating entry in it's active_view for node {}", peer, addr)),
                },
                None => assert!(false, format!("really fouled up, couldn't get known peer {} from master set of nodes :(", peer)),
            }
        }
    }

//    assert!(false, "***** DUMMY FALSE ASSERT *****");
}
