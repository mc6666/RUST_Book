#![allow(unused)]

use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use futility_try_catch::try_;
 
fn read_file(file_path:String) -> Result<String, Box<dyn Error>> {
    let mut contents: String="".to_string();
    try_!({
        // 建立檔案路徑
        let path = Path::new(&file_path);

        // 開啟檔案
        let mut file = File::open(&path)?;

        // 讀取檔案內容
        contents = String::new();
        file.read_to_string(&mut contents)?;
    } catch Box<dyn Error> as err {
        panic!("讀取檔案內容錯誤 !!")
    });

    // 回傳檔案內容
    Ok(contents)
}

fn main() {
    let contents = match read_file("data.txt".to_string()) {
        Err(error) =>println!("{}", error),
        Ok(contents) => println!("contents:{}", contents)
    };
}
