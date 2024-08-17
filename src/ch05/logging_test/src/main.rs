#![allow(unused_imports)]
use log::{error, warn, info, debug, trace};
use env_logger;

fn main() {
    env_logger::init();
    trace!("trace message");
    debug!("debug message");
    info!("info message");
    warn!("warn message");
    error!("error message");
}
