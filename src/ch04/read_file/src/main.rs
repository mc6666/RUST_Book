use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

fn main() {
    // 建立檔案路徑
    let path = Path::new("data.txt");
    let display = path.display();

    // 開啟檔案
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // 讀取檔案內容
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
}
