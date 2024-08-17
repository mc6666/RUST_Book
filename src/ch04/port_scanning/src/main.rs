#![allow(unused)]

use std::{env, io};
use std::io::{Error, Result};
use std::net::{TcpListener, TcpStream};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Port {
    pub port: u16,
}

impl Port {
    fn new(new_port: u16) -> Port {
        Port { port: new_port }
    }

    fn is_available(&self) -> Result<bool> {
        TcpListener::bind(("127.0.0.1", self.port)).map(|_| true)
    }

    fn is_in_use(&self) -> Result<bool> {
        TcpStream::connect(("127.0.0.1", self.port)).map(|_| true).map_err(|err| Error::new(io::ErrorKind::Other, format!("Error connecting to port: {}", err))) // Handle connection errors more informatively
    }
}

fn main() -> Result<()> {
    let default_port = 3000;
    let mut port = default_port;


    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No port number specified, using default port: {}", port);
    } else {
        match args[1].parse::<u16>() {
            Ok(value) => port = value,
            Err(err) => return Err(Error::new(io::ErrorKind::InvalidInput, format!("Invalid port number: {}", err))),
        }
    }

    let port_obj = Port::new(port);
    println!("available：{:?}", port_obj.is_available());
    // println!("in use：{:?}", port_obj.is_in_use());

    match port_obj.is_available().and_then(|available| port_obj.is_in_use().map(|in_use| (available, in_use))) {
        Ok((true, true)) => println!("Port {}: Available and in use", port),
        Ok((true, false)) => println!("Port {}: Available but not in use", port),
        Ok((false, _)) => println!("Error binding to Port {}: ", port),
        Err(err) => { //return Err(err),
            match err.kind() {
                std::io::ErrorKind::Other => println!("in use：{:?}", err.to_string()),
                _ => println!("Other error: {:?}", err)
            }
        }
    }

    Ok(())
}