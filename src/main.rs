extern crate config;
extern crate getopts;
extern crate hyparview;

use config::Config;
use getopts::{optopt,optflag,getopts,OptGroup,usage};
use hyparview::messages::{deserialize,HyParViewMessage};
use std::io::{TcpListener,TcpStream,Acceptor,Listener};
use std::os;
use std::sync::Arc;
use std::thread::Thread;
use std::sync::mpsc::{channel,Sender};

fn main() {
    println!("starting polaris");
    let opts = Opts::read_opts();
    let config = box Config::load_config(opts.config_file.as_slice());
    let config_arc = Arc::new(*config);
    let (tx, rx) = channel::<HyParViewMessage>();

    // TODO: init hyparview *after* binding to the socket
    hyparview::start_service(config_arc.clone(), rx);

    let config_cpy = config_arc.clone();
    println!("going to bind to addr: {}", config_cpy.local_addr);
    let listener = TcpListener::bind(config_cpy.local_addr);
    let mut acceptor = listener.listen();
    for conn in acceptor.incoming() {
        println!("aceepting an incoming connection!");
        let tx = tx.clone();
        match conn {
            Err(e) => println!("failure with acceptor: {}", e),
            // TODO: this builds a new thread per client, maybe just want some TaskPool/handler instead - or mio (https://github.com/carllerche/mio)
            Ok(conn) => Thread::spawn(move || {
                let conn = conn.clone();
                handle_client(conn, tx);
            }).detach(),
        }
    }
}

fn handle_client(mut stream: TcpStream, sender: Sender<HyParViewMessage>) {
    let addr = stream.peer_name().ok().expect("failed to get the remote peer addr from an open socket.");
    println!("got a connection from {}", addr);
    let res = hyparview::messages::deserialize(&mut stream);
    if res.is_ok() {
        match sender.send(res.unwrap()) {
            Ok(_) => {},
            Err(e) => println!("failed to send task:{}", e),
        };
    } else {
        //TODO: learn to print out the err message
        println!("failed to parse incoming message");
    }
    // TODO send a 'socket closed' event to the hyparview controller
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
