#![allow(unused)]

use reqwest;

async fn fetch_url(url:&str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url)
                    .await?.text().await?;
    Ok(response)                
}

#[tokio::main]
async fn main() {
    // 正確網址：https://www.rust-lang.org
    let response = fetch_url("https://abcdefxyz.org").await;
    if response.is_ok() {
        println!("{:?}", response);
    } else {
        println!("無此網頁: {:?}", response.err().unwrap());
    }   
}
