extern crate hyparview;

use std::collections::BTreeMap;
use std::io::{TcpListener,TcpStream,BufferedReader};
use std::io::{Acceptor,Listener};
use std::io::File;
use std::io::net::ip::{Ipv4Addr, SocketAddr};
use std::str;
use std::thread::Thread;

// heavily influenced by hyper HTTP lib: https://github.com/hyperium/hyper/blob/master/src/server/mod.rs
fn main() {
    println!("stating polaris");
    let config = Config::load_config("/opt/dev/projects/rust/polaris/config/config.toml".as_slice());

    // set up some networking infra

    // kick off hyparview, get back a channel 
    hyparview::start_service(config.local_socket, config.seeds);

    //listen for messages and pass them over the channel

    let listener = TcpListener::bind("127.0.0.1:9090");
    let mut acceptor = listener.listen();
    let mut captured = acceptor.clone();

    for stream in acceptor.incoming() {
        match stream {
            Err(e) => println!("failure with acceptor: {}", e),
            Ok(stream) => Thread::spawn(move || {
                let stream = stream.clone();
                handle_client(stream);
            })
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    println!("hello client");
    // deserialize message 
    Ok
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
        let mut seeds : Vec<SocketAddr>;
        seeds.push(from_str("127.0.0.1:9090").expect("malformed address"));
        seeds.push(from_str("127.0.1.1:9090").expect("malformed address"));
//        seeds.push(SocketAddr = from_str("127.0.0.1:9090").expect("malformed address"));
        Config { local_addr: local_addr, seeds: seeds }
    }
}

