use rev_lines::RevLines;
use std::io::BufReader;

fn main() {
    let path = std::env::args().nth(1).expect("請提供檔名.");
    let n:u64;
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

    // read file reversely
    let mut reader = RevLines::new(BufReader::new(f));

    // save last n rows into vec
    let mut vec: Vec<String> = Vec::new();
    for _ in 0..n {
        // 設定讀取列數超過檔案列數，reader.next() 會回傳 None
        let _ = match reader.next() {
            Some(value) => vec.push(format!("{}", value.unwrap())),
            None => ()
        };
    }
    
    // print last n rows
    for line2 in vec.iter().rev() {
        println!("{line2}");
    }
}
