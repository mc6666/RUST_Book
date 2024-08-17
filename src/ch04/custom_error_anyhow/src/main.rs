use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use anyhow::{Context, Result};

fn read_file(file_path:String) -> Result<String> {
    // 建立檔案路徑
    let path = Path::new(&file_path);

    // 開啟檔案
    let mut file = File::open(&path)
        .with_context(|| format!("(E1101) 開啟檔案 {} 錯誤"
                , file_path))?;

    // 讀取檔案內容
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .with_context(|| format!("(E1102) 讀取檔案 {} 內容錯誤"
            , file_path))?;
    
    // 回傳檔案內容
    Ok(contents)
}

fn main() {
    let _ = match read_file("data.txt".to_string()) {
        Err(error) => panic!("{}", error),
        Ok(contents) => println!("contents:{}", contents)
    };
}
