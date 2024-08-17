#![allow(unused_imports)]
use log::*;
use env_logger;
use std::fs::File;
use std::io::Write;
use chrono::Local;
use std::fs::OpenOptions;

fn main() {
    
    // let target = Box::new(File::create("log.txt").expect("Can't create file"));
    let target = Box::new(OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .append(true)
                .open("log.txt").expect("Can't create file"));

    // env_logger::Builder::from_env(env_logger::Env::default()).init();
    let mut builder = env_logger::Builder::from_default_env();
    builder
        .target(env_logger::Target::Pipe(target))
        .filter(None, LevelFilter::Debug)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();

    // env_logger::Builder::new()
        // .target(env_logger::Target::Pipe(target))
        // .filter(None, LevelFilter::Debug)
        // .format(|buf, record| {
            // writeln!(
                // buf,
                // "[{} {} {}:{}] {}",
                // Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                // record.level(),
                // record.file().unwrap_or("unknown"),
                // record.line().unwrap_or(0),
                // record.args()
            // )
        // })
        // .init();

    trace!("trace message");
    debug!("debug message");
    info!("info message");
    warn!("warn message");
    error!("error message");
}
