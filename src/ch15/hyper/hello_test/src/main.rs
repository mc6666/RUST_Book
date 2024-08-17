#![allow(unused)]

use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::body::Bytes;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use tokio::net::TcpListener;
use hyper_util::rt::TokioIo;

// 回傳 Hello World!
async fn hello(_: Request<impl hyper::body::Body>) -> 
        Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello World!"))))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 開啟工作日誌
    pretty_env_logger::init();

    // 設定本機網址及通訊埠
    let addr: SocketAddr = ([127, 0, 0, 1], 8000).into();

    // 監聽連線
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        // 接受連線請求
        let (tcp, _) = listener.accept().await?;
        // Tokio 初始化 
        let io = TokioIo::new(tcp);

        // 生成一個新的執行緒處理連線請求
        tokio::task::spawn(async move {
            // 呼叫 hello 函數
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(hello))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}