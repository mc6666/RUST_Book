#![allow(unused)]

// use std::{
    // thread,
    // time::Duration,
// };
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::fs;
use tokio::io::{self, BufReader, AsyncBufReadExt};
use tokio::io::AsyncReadExt; // for read_to_end()

#[tokio::main]
async fn main() {
    // 建立 TCP Listener， 監聽 8000 埠(port)
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();

    // 接收到請求，由 handle_connection 處理
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        
        tokio::spawn(async {
            handle_connection(stream).await;
        });
    }
}

// 檢查檔案是否存在
async fn check_file_exist(path: &str) -> io::Result<bool> {
    let file = fs::File::open(path).await?; // 開啟檔案
    let metadata = file.metadata().await?;  // 讀取檔案屬性
    Ok(metadata.is_file()) // 以上執行成功，則回傳是否為檔案，而非目錄
}

// 處理連線請求
async fn handle_connection(mut stream: TcpStream) {
    // 讀取請求表頭第1行
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line).await;
    
    // 讀取網址
    let mut file_name = "".to_string();
    let mut status_line = "";
    let mut vec = request_line.as_str().split(" "); // 以空白切割字串
    let mut uri = vec.next().unwrap(); // 取第1個分割字串
    uri = vec.next().unwrap();         // 取第2個分割字串
    
    // 檔案名稱："." + 第1個分割字串 + ".html"
    let mut uri_full: String = ".".to_string() + uri;   
    // if !uri_full.ends_with(".html") {uri_full = uri_full + ".html"}; 
    println!("{}", uri_full);
    
    // 準備回應內容及檔案
    if uri == "/".to_string() { // 讀取 index.html 內容
        status_line = "HTTP/1.1 200 OK";
        file_name = "index.html".to_string();
    } else if let Ok(b) = check_file_exist(&uri_full).await {
        status_line = "HTTP/1.1 200 OK";
        file_name = uri_full;
    } else { // 讀取 404.html 內容
        status_line = "HTTP/1.1 404 NOT FOUND";
        file_name = "404.html".to_string();
    }

    // 計算結果長度
    let mut contents = vec![];
    let mut file = fs::File::open(file_name).await.unwrap();
    file.read_to_end(&mut contents).await;
    let length = contents.len();
    
    // 回應格式
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");
    
    // 等待 5 秒
    // thread::sleep(Duration::from_secs(5));
    
    // 回傳結果
    stream.try_write(response.as_bytes()).unwrap();
    stream.try_write(&contents).unwrap();
}
