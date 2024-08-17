#![allow(unused)]

use std::error::Error;
use serde::Deserialize;
use csv;

#[derive(Deserialize, Debug)] // 反序列化，可自動解析欄位
#[allow(non_snake_case)] // 允許大寫欄位名稱
struct Record {
    Country: String,
    Region: String,
}

// read file
fn read_csv(path: &String) -> Result<Vec<Record>, Box<dyn Error>> {
    let file = std::fs::File::open(&path)?;
    let mut reader = csv::Reader::from_reader(file);
    let mut vec:Vec<Record> = Vec::new();
    for record in reader.deserialize() {
        println!("{:?}", record);
        let record: Record = record?;
        vec.push(record);
    }

    Ok(vec)  // 成功就回傳檔案內容
}

fn main() {
    // 讀取命令行參數
    let path = std::env::args().nth(1).expect("未指明檔案路徑 !!");  
    // 讀取檔案內容及錯誤處理
    let mut vec:Vec<Record> = Vec::new();
    let _ = match read_csv(&path) {
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
