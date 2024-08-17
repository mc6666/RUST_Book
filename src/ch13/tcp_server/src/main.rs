#![allow(unused)]

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // 建立 TCP Listener， 監聽 8000 埠(port)
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    // 接收到請求，由 handle_connection 處理
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    const BUFFER_LEN: usize = 512;
    let mut buffer = [0u8; BUFFER_LEN];
    let mut contents = vec![];
    loop {
        let read_count = buf_reader.read(&mut buffer).unwrap();
        contents.extend_from_slice(&buffer[..read_count]);

        if read_count != BUFFER_LEN {
            break;
        }
    }
    println!("{:?}", contents);
}