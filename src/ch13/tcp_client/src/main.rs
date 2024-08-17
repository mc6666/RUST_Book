use std::io::prelude::*;
use std::io::BufWriter;
use std::net::TcpStream;

fn main() {
    let mut stream = BufWriter::new(
            TcpStream::connect("127.0.0.1:8000").unwrap());

    for i in 0..10 {
        stream.write(format!("{}", i).as_bytes()).unwrap();
    }
    stream.flush().unwrap();
}
