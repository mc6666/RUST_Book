#![allow(unused)]

use std::thread;
use std::error::Error;
use reqwest;
use std::time::{Duration, Instant};
use std::io::Read;
use strfmt::strfmt;
use std::collections::HashMap;
use encoding::{Encoding, EncoderTrap, DecoderTrap};

// 網址
const URL_TEMPLATE: &str = "http://www.twse.com.tw/exchangeReport/MI_INDEX?response=csv&date={date1}&type=ALL";

async fn get_data(date: &str) -> Result<(), Box<dyn Error>> {
    let mut date1 = date.to_string();
    let mut vars = HashMap::new();
    vars.insert("date1".to_string(), date1.clone());
    let url: String = strfmt(&URL_TEMPLATE, &vars).unwrap();
    let body = reqwest::get(url)
        .await?
        .bytes()
        .await?;

    let decoding_data = encoding::all::BIG5_2003.decode(&body, DecoderTrap::Strict).unwrap();
    // println!("{}", decoding_data);
    std::fs::write(date1 + ".txt", decoding_data)?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let dates = vec!["20240304", "20240305", "20240306", "20240307", "20240308", ];
    
    // 計時開始
    let start = Instant::now();

    let mut vec_handle = vec![];
    for date1 in dates {
        vec_handle.push(tokio::spawn(async {
            get_data(date1).await;
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