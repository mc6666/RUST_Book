#![allow(unused)]

use std::error::Error;
use serde::Deserialize;
use csv;
use reqwest;

#[derive(Deserialize, Debug)] // 反序列化，可自動解析欄位
#[allow(non_snake_case)] // 允許大寫欄位名稱
struct Record {
    Country: String,
    Region: String,
}

async fn fetch_url(url: &str) -> Result<String, reqwest::Error>  {
    let content = reqwest::get(url).await.unwrap().text().await;
    // println!("{:?}", content);
    content
}

// read file
async fn read_csv(url: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut content:String="".to_string();
    let _ = match fetch_url(&url).await {
        Err(error) => println!("{}", error),
        Ok(content1) => content = content1
    };
    // println!("{:?}", content);
    
    let mut reader = csv::Reader::from_reader(content.as_bytes());
    let mut vec:Vec<Record> = Vec::new();
    for record in reader.deserialize() {
        println!("{:?}", record);
        let record: Record = record?;
        vec.push(record);
    }

    Ok(vec)  // 成功就回傳檔案內容
}

#[tokio::main]
async fn main() {
    // 讀取命令行參數
    let path = std::env::args().nth(1).expect("未指明檔案路徑 !!");  
    // let path = "https://raw.githubusercontent.com/cs109/2014_data/master/countries.csv";
    // 讀取檔案內容及錯誤處理
    let mut vec:Vec<Record> = Vec::new();
    let _ = match read_csv(&path).await {
        Err(error) => println!("{}", error),
        Ok(content) => {vec = content;}
    };
    // 篩選北美的國家
    vec = vec.into_iter().filter(|x| x.Region == "NORTH AMERICA").collect();
    println!("\nAfter filter：");
    for record in vec {
        println!("{:?}", record);
    }
}
