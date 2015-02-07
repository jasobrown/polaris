extern crate log;
extern crate time;

use log::{Log,LogLevel,LogLevelFilter,LogRecord,set_logger};
use time::{now,strftime};

//use std::old_io::net::ip::{SocketAddr};
//use std::old_io::{File, Open, Write,USER_RWX};
//use std::os::tmpdir;
//use std::old_io::fs::{mkdir_recursive,PathExtensions};
//use std::sync::mpsc::{channel,Sender,Receiver};
use std::thread::Thread;
//use std::old_io::{LineBufferedWriter,stdio,stderr};
//use std::old_io::{stderr};

// based on http://joshitech.blogspot.com/2014/12/rust-customer-logger.html

pub fn start_service() {
    match set_logger(|max_log_level| {
        //TODO: actually get real log level from somwhere
        let log_level = LogLevelFilter::Debug;
        max_log_level.set(log_level);
        Box::new(LocalLogger::new(LogLevel::Debug))
    }) {
        Ok(s) => info!("polaris logging init'd: {:?}", s),
        Err(e) => error!("polaris logging could not be initialized: {:?}", e),
    }
}

#[derive(Copy)]
pub struct LocalLogger {
    log_level: LogLevel,
}
impl LocalLogger {
    pub fn new(log_level: LogLevel) -> LocalLogger { 
        LocalLogger { log_level: log_level }
    }
}
impl Log for LocalLogger {
    fn log(&self, record: &LogRecord) {
        let thr = Thread::current();
        let thread_name = match thr.name() {
            Some(n) => n,
            None => "unnamed thread".as_slice(),
        };

        println!("{} [{}] {} {}:{} (line {}) {}",
                        time::strftime("%Y-%m-%d %H:%M:%S.%f %Z", &time::now()).unwrap(),
                        thread_name,
                        record.level(),
                        record.location().module_path,
                        record.location().file,
                        record.location().line,
                        record.args());
    }

    #[allow(unused_variables)]
    fn enabled(&self, level: LogLevel, module: &str) -> bool {
        return level <= self.log_level;
    }
}

// struct LogWriter {
//     file: Box<File>,
// }
// impl LogWriter {
//     fn new(config: &Config) -> LogWriter {
//         let mut p = tmpdir().clone();
//         p.push("polaris");
//         if !p.exists() {
//             match mkdir_recursive(&p, USER_RWX) {
//                 Ok(_) => {}
//                 Err(e) => panic!("failed to create tmp dir: {}", e),
//             };
//         }
    
//         p.push(format!("{}", config.local_addr));
//         let mut file = match File::open_mode(&p, Open, Write) {
//             Ok(f) => f,
//             Err(e) => panic!("file error: {}", e),
//         };

//         LogWriter { file: file }
//     }

//     fn listen(&mut self, receiver: Receiver<String>) {
//         match receiver.iter() {
//             Some(rec) => self.file.write(rec),
//             None => return,
//         }
//     }
// }
