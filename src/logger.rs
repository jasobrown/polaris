//#[phase(plugin, link)]extern crate log;  
//#[phase(plugin, link)]extern crate time;
extern crate time;

use config::Config;
use log::{Logger,LogRecord};
use std::io::{File, Open, Write,USER_RWX};
use std::os::tmpdir;
use std::io::fs::{mkdir_recursive,PathExtensions};
use time::{now,strftime};

// based on http://joshitech.blogspot.com/2014/12/rust-customer-logger.html
pub struct LocalLogger {
    file: File,
}
impl LocalLogger {
    pub fn new(config: &Config) -> LocalLogger {
        let mut p = tmpdir().clone();
        p.push("polaris");
        if !p.exists() {
            match mkdir_recursive(&p, USER_RWX) {
                Ok(_) => {}
                Err(e) => panic!("failed to create tmp dir: {}", e),
            };
        }


        p.push(format!("{}", config.local_addr));
//        info!("will log at file {}", p);
        let file = match File::open_mode(&p, Open, Write) {
            Ok(f) => f,
            Err(e) => panic!("file error: {}", e),
        };

        LocalLogger { file: file }
    }
}
impl Logger for LocalLogger {
    fn log(&mut self, record: &LogRecord) {
        match writeln!(&mut self.file,
                       "{} {} {}:{} (line {}) {}",
                       time::strftime("%Y-%m-%d %H:%M:%S.%f %Z", &time::now()).unwrap(),
                       record.level,
                       record.module_path,
                       record.file,
                       record.line,
                       record.args) {
            Ok(()) => {}
            Err(e) => println!("failed to log: {}", e),
        }
    }  
}
