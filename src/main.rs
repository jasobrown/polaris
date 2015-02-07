#![feature(slicing_syntax)]
#![feature(env)]
#![feature(io)]
#![feature(core)]
#![feature(std_misc)] 

#[macro_use] extern crate log;
#[macro_use] extern crate time;

extern crate getopts;
extern crate polaris;

use getopts::{Options};
use polaris::config::Config;
use polaris::hyparview::messages::{deserialize,HyParViewMessage};
use polaris::logger;
use std::old_io::{TcpListener,TcpStream,Acceptor,Listener};
use std::env;
use std::sync::Arc;
use std::thread::Thread;
use std::sync::mpsc::{Sender};

fn main() {
    logger::start_service();
    debug!("starting polaris");
    let opts = Opts::read_opts();
    let config = Box::new(Config::load_config(opts.config_file.as_slice()));

    let config_arc = Arc::new(*config);
    let config_cpy = config_arc.clone();
    info!("binding to local addr: {}", config_cpy.local_addr);
    let listener = TcpListener::bind(config_cpy.local_addr);
    let mut acceptor = listener.listen();

    let sender = polaris::hyparview::start_service(config_arc.clone());

    for conn in acceptor.incoming() {
        if conn.is_ok() {
            // TODO: this builds a new thread per client, maybe just want some TaskPool/handler instead - or mio (https://github.com/carllerche/mio)
            let sender = sender.clone();
            Thread::spawn(move || {
                let conn = conn.unwrap().clone();
                handle_client(conn, sender);
            });
        } else {
            error!("failure with acceptor");
        }
    }

    polaris::plumtree::start_service();
}

fn handle_client(mut stream: TcpStream, sender: Sender<HyParViewMessage>) {
    match polaris::hyparview::messages::deserialize(&mut stream) {
        Ok(msg) => {
            match sender.send(msg) {
                Ok(_) => {},
                Err(_) => error!("failed to send task"),
            };
        },
        Err(e) => error!("failed to parse incoming message: {}", e),
    }
    // TODO send a 'socket closed' event to the hyparview controller
}

/// struct to hold the parsed command line args to the program.
struct Opts {
    config_file: String,
}
impl Opts {
    fn read_opts() -> Opts {
        let args = env::args();
        let mut opts = Options::new();
        opts.optopt("c", "config", "(required) path to the central configuration file", "");
        opts.optflag("h", "help", "print this help menu");

        let matches = match opts.parse(args) {
            Ok(m) => { m }
            Err(f) => { panic!("{:?}", f) }
        };
        if matches.opt_present("h") {
            Opts::print_usage(opts);
            // TODO: a more elegant way to exit to program
            panic!("exiting after help");
        }
        
        let config_file = match matches.opt_str("c") {
            Some(x) => x,
            None => panic!("must pass in a location of configuration file (-c FILE)"),
        };

        Opts { config_file : config_file }
    }

    fn print_usage(opts: Options) {
        let brief = format!("Usage: [options]");
        println!("{}", opts.usage(brief.as_slice()));
    }
}
