use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::fmt;

struct MyError(String);
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn read_file(file_path:String) -> Result<String, MyError> {
    // 建立檔案路徑
    let path = Path::new(&file_path);

    // 開啟檔案
    let mut file = File::open(&path)
        .map_err(|err| MyError(format!("(E1101) 開啟檔案 {} 錯誤: {}"
                , file_path, err)))?;

    // 讀取檔案內容
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|err| MyError(format!("(E1102) 讀取檔案 {} 內容錯誤: {}"
            , file_path, err)))?;
    
    // 回傳檔案內容
    Ok(contents)
}

fn main() {
    let _ = match read_file("data.txt".to_string()) {
        Err(error) => panic!("{}", error),
        Ok(contents) => println!("contents:{}", contents)
    };
}
