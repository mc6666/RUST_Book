#![allow(unused)]

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use threadpool::ThreadPool;

fn main() {
    // 建立 TCP Listener， 監聽 8000 埠(port)
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    // 接收到請求，由 handle_connection 處理
    let pool = ThreadPool::new(10); // 最大的執行緒服務數量=10
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        // thread::spawn(|| {
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
        // .lines()
        // .map(|result| result.unwrap())
        // .take_while(|line| !line.is_empty())
        // .collect();

    // println!("Request: {:#?}", http_request);
    
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let mut file_name = "";
    let mut status_line = "";
    match request_line.as_str() {
        "GET / HTTP/1.1" | "GET /index.html HTTP/1.1" => {
            // 準備回應內容
            status_line = "HTTP/1.1 200 OK";
            // 讀取 index.html 內容
            file_name = "index.html";
            },
        "GET /register.html HTTP/1.1" => {
            // 準備回應內容
            status_line = "HTTP/1.1 200 OK";
            // 讀取 index.html 內容
            file_name = "register.html";
            },
        _ => {
            // 準備回應內容：404 無此網頁
            status_line = "HTTP/1.1 404 NOT FOUND";
            // 讀取 404.html 內容
            file_name = "404.html";
            },
    }
    let contents = fs::read_to_string(file_name).unwrap();
    // 計算 index.html 內容長度
    let length = contents.len();
    // 回應要求
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    // 等待 5 秒
    thread::sleep(Duration::from_secs(5));
    
    stream.write_all(response.as_bytes()).unwrap();
}
