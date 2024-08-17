#![allow(unused)]

use std::error::Error;
use reqwest;
use strfmt::strfmt;
use std::collections::HashMap;
use encoding::{Encoding, EncoderTrap, DecoderTrap};

// 網址
const URL_TEMPLATE: &str = "http://www.twse.com.tw/exchangeReport/MI_INDEX?response=csv&date={date1}&type=ALL";

async fn get_data(date1: String) -> Result<(), Box<dyn Error>> {
    // 格式化網址
    let mut vars = HashMap::new();
    vars.insert("date1".to_string(), date1.clone());
    let url: String = strfmt(&URL_TEMPLATE, &vars).unwrap();

    // 每日收盤行情 BIG5 內碼，需使用 bytes
    let body = reqwest::get(url)
        .await?
        .bytes()
        .await?;

    // bytes 轉換為 Big5字串
    let decoding_data = encoding::all::BIG5_2003.decode(&body, 
                            DecoderTrap::Strict).unwrap();
    println!("{}", decoding_data);
    // std::fs::write 寫入的檔案預設內碼是UTF-8
    std::fs::write(date1 + ".txt", decoding_data)?;

    Ok(())
}

#[tokio::main]
async fn main() {
    // 下載特定日期的股價
    get_data("20240308".to_string()).await;
}