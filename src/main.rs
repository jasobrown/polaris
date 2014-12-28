extern crate getopts;
extern crate hyparview;

use getopts::{optopt,optflag,getopts,OptGroup,usage};
use std::io::{TcpListener,TcpStream,Acceptor,Listener};
use std::os;
use std::thread::Thread;

mod config;

// heavily influenced by hyper HTTP lib: https://github.com/hyperium/hyper/blob/master/src/server/mod.rs
fn main() {
    println!("starting polaris");
    let opts = Opts::read_opts();
    let config = config::Config::load_config(opts.config_file.as_slice());
    let hpv_sender = hyparview::start_service(config.local_addr, config.contact_nodes);

    let listener = TcpListener::bind(config.local_addr);
    let mut acceptor = listener.listen();
    for conn in acceptor.incoming() {
        println!("aceepting an incoming!");
        let hpv_sender = hpv_sender.clone();
        match conn {
            Err(e) => println!("failure with acceptor: {}", e),
            // TODO: this builds a new thread per client, maybe just want some TaskPool/handler instead - or mio (https://github.com/carllerche/mio)
            Ok(conn) => Thread::spawn(move || {
                let conn = conn.clone();
                handle_client(conn, hpv_sender);
            }).detach(),
        }
    }
}

fn handle_client(mut stream: TcpStream, sender: Sender<hyparview::messages::HyParViewMessage>) {
    println!("hello client");
    match hyparview::messages::deserialize(&mut stream) {
        Ok(msg) => sender.send(msg),
        Err(e) => println!("failed to parse incoming message: {}", e),
    }
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
            optopt("c", "config", "(required) path to the central configuration file", ""),
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
            None => panic!("must pass in a location of configuration file (-c FILE)"),
        };

        Opts { config_file : config_file }
    }

    fn print_usage(program: &str, opts: &[OptGroup]) {
        let brief = format!("Usage: {} [options]", program);
        print!("{}", usage(brief.as_slice(), opts));
    }
}
