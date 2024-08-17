use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let path = std::env::args().nth(1).expect("請提供檔名.");
    let n:i32;
    if std::env::args().len() <= 2 {
        n = 5;
    } else {
        n = std::env::args().nth(2).expect("請提供讀取列數.")
            .trim().parse().expect("請提供讀取列數.");
    }
    
    // check file exist?
    if !std::path::Path::new(&path).exists() {
        panic!("檔案 {path} 不存在.");
    }

    // open file
    let f = std::fs::File::open(&path).expect(&format!("無法讀取檔案 {}.", path));
    let mut reader = BufReader::new(f);

    // read file
    let mut line = String::new();
    for _ in 0..n {
        let _ = reader.read_line(&mut line).expect("");
        print!("{line}");
        line = String::new(); // reset line
    }
}
