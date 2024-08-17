#![allow(unused)]

use std::thread;
use std::error::Error;
use reqwest;
use std::time::{Duration, Instant};
use std::io::Read;

async fn get_data() -> Result<String, Box<dyn Error>> {
    let body = reqwest::get("http://localhost:8000")
        .await?
        .text()
        .await?;

    // println!("Status: {}", res.status());
    // println!("Headers:\n{:#?}", res.headers());
    // println!("Body:\n{}", body);

    Ok(body)
}

#[tokio::main]
async fn main() {
    let n_jobs = 10000;
    
    // 計時開始
    let start = Instant::now();

    let mut vec_handle = vec![];
    for _ in 0..n_jobs {
        vec_handle.push(tokio::spawn(async {
            get_data().await;
            // let body = match get_data().await {
                // Ok(content) => println!("Body:{}", content),
                // Err(err) => println!("{}", err),
            // };
        }));
    }
    // println!("Count:{}", vec_handle.len());
    
    // 保證所有執行緒會執行完畢，才結束主程式
    for handle in vec_handle {
        handle.await;
    }
    
    let duration = start.elapsed();
    println!("耗時: {:?}", duration);

}