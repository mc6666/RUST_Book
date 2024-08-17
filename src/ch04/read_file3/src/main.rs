#![allow(unused)]

use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

fn read_file(file_path:String) -> Result<String, std::io::Error> {
    // 建立檔案路徑
    let path = Path::new(&file_path);

    // 開啟檔案
    let mut file = File::open(&path)?;

    // 讀取檔案內容
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    // 回傳檔案內容
    Ok(contents)
}

fn main() {
    let contents = match read_file("data.txt".to_string()) {
        Err(error) => println!("{}", error),
        Ok(contents) => println!("contents:{:?}", contents)
    };
}
