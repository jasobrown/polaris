extern crate hyparview;

use std::io::{TcpListener,TcpStream};
use std::io::{Acceptor,Listener};
use std::io::net::ip::{SocketAddr};
use std::thread::Thread;

// heavily influenced by hyper HTTP lib: https://github.com/hyperium/hyper/blob/master/src/server/mod.rs
fn main() {
    println!("stating polaris");
    let config = Config::load_config("/opt/dev/projects/rust/polaris/config/config.toml".as_slice());
    let sender = hyparview::start_service(config.local_addr, config.seeds);

    let listener = TcpListener::bind("127.0.0.1:9091");
    let mut acceptor = listener.listen();
    for conn in acceptor.incoming() {
        println!("aceepting an incoming!");
        let sender = sender.clone();
        match conn {
            Err(e) => println!("failure with acceptor: {}", e),
            // TODO: this builds a new thread per client, maybe just want some TaskPool/handler instead - or mio (https://github.com/carllerche/mio)
            Ok(conn) => Thread::spawn(move || {
                let conn = conn.clone();
                handle_client(conn, sender);
            }).detach(),
        }
    }
}

fn handle_client(mut stream: TcpStream, sender: Sender<int>) {
    println!("hello client");
    // TODO: deserialize message 

    sender.send(42);
}

struct Config {
    local_addr: SocketAddr,
    seeds: Vec<SocketAddr>,
}
impl Config {
    fn load_config(file_name: &str) -> Config {
        // let path = Path::new(file_name);
        // let mut reader = BufferedReader::new(File::open(&path));

        // for line in reader.lines() {
        //     let split: Vec<&str> = line.as_slice().split(':').collect();
        //     props.insert(split[0], split[1]);
        // }

        // let local_addr = match props.get {
        //     Some(addr) => SocketAddr::from_str(addr),
        //     None => SocketAddr::from_str("127.0.0.1:9090"),
        // };

        let local_addr : SocketAddr = from_str("127.0.0.1:9090").expect("malformed address");
        let mut seeds = Vec::new();
        seeds.push(from_str("127.0.0.1:9090").expect("malformed address"));
        seeds.push(from_str("127.0.1.1:9090").expect("malformed address"));
        Config { local_addr: local_addr, seeds: seeds }
    }
}

