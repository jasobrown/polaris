#![feature(slicing_syntax)]

#[macro_use] extern crate log;
#[macro_use] extern crate time;

extern crate getopts;

use config::Config;
use getopts::{optopt,optflag,getopts,OptGroup,usage};
use hyparview::messages::{deserialize,HyParViewMessage};
use log::set_logger;
use logger::LocalLogger;
use std::io::{TcpListener,TcpStream,Acceptor,Listener};
use std::os;
use std::sync::Arc;
use std::thread::Thread;
use std::sync::mpsc::{Sender};

mod config;
mod hyparview;
mod logger;

fn main() {
    let opts = Opts::read_opts();
    let config = Box::new(Config::load_config(opts.config_file.as_slice()));

    let config_arc = Arc::new(*config);
    let config_cpy = config_arc.clone();
    info!("binding to local addr: {}", config_cpy.local_addr);
    let listener = TcpListener::bind(config_cpy.local_addr);
    let mut acceptor = listener.listen();

    let sender = hyparview::start_service(config_arc.clone());

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
}

fn handle_client(mut stream: TcpStream, sender: Sender<HyParViewMessage>) {
    match hyparview::messages::deserialize(&mut stream) {
        Ok(msg) => {
            match sender.send(msg) {
                Ok(_) => {},
                Err(e) => error!("failed to send task"),
            };
        },
        Err(e) => error!("failed to parse incoming message"),
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
