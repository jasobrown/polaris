extern crate getopts;
extern crate hyparview;

use getopts::{optopt,optflag,getopts,OptGroup,usage};
use std::io::{TcpListener,TcpStream,BufferedReader,Acceptor,Listener};
use std::io::fs::File;
use std::io::net::ip::{SocketAddr};
use std::os;
use std::thread::Thread;

// heavily influenced by hyper HTTP lib: https://github.com/hyperium/hyper/blob/master/src/server/mod.rs
fn main() {
    println!("stating polaris");
    let opts = Opts::read_opts();
    let config = Config::load_config(opts.config_file.as_slice());
    let sender = hyparview::start_service(config.local_addr, config.contact_nodes);

    let listener = TcpListener::bind(config.local_addr);
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

/// struct to hold the parsed command line args to the program.
struct Opts {
    config_file: String,
}
impl Opts {
    fn read_opts() -> Opts {
        let args: Vec<String> = os::args();
        let program = args[0].clone();

        let opts = &[
            optopt("c", "config_file", "(required) path to the central configuration file", ""),
            optflag("h", "help", "print this help menu")
        ];
        let matches = match getopts(args.tail(), opts) {
            Ok(m) => { m }
            Err(f) => { panic!(f.to_string()) }
        };
        if matches.opt_present("h") {
            Opts::print_usage(program.as_slice(), opts);
            // TODO: a more elegant way to exit to program
            panic!("exiting after help");
        }
        
        let config_file = match matches.opt_str("c") {
            Some(x) => x,
            None => panic!("must pass in a location of conifuratioon file"),
        };

        Opts { config_file : config_file }
    }

    fn print_usage(program: &str, opts: &[OptGroup]) {
        let brief = format!("Usage: {} [options]", program);
        print!("{}", usage(brief.as_slice(), opts));
    }
}

struct Config {
    pub local_addr: SocketAddr,
    pub contact_nodes: Vec<SocketAddr>,
}
impl Config {
    /// i really don't want to create a toml/yaml/whatever lib or pull in one (for now), so just use a 
    /// fixed format text file. current format is (each line, that is):
    /// - local_addr: ipAddrV4:port
    /// - contact_nodes: comma delimited list of {ipAddrV4:port} tuples
    fn load_config(file_name: &str) -> Config {
        let path = Path::new(file_name);
        let mut reader = BufferedReader::new(File::open(&path));

        let line = reader.read_line().ok().expect("Failed to read line");
        let local_addr: SocketAddr = from_str(line.as_slice().trim()).expect("malformed address");

        let mut contact_nodes = Vec::new();
        let line = reader.read_line().ok().expect("Failed to read line");
        let v: Vec<&str> = line.split_str(",").collect();
        for addr in v.iter() {
            contact_nodes.push(from_str(addr.as_slice().trim()).expect("malformed address"));
        }

        Config { local_addr: local_addr, contact_nodes: contact_nodes }
    }
}

