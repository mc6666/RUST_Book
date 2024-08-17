use std::{error::Error, fs::File, process};

// read csv
fn get_csv_content(path: &String) -> Result<(), 
                    Box<dyn Error>> {
    let file = File::open(&path)?;  // 開啟檔案
    let mut rdr = csv::Reader::from_reader(file); // 建立 CSV reader
    for result in rdr.records() {
        let record = result?;      // 檢查錯誤
        println!("{:?}", record);  // 顯示一筆資料
    }
    Ok(())
}

fn main() {
    // 讀取命令行參數
    let path = std::env::args().nth(1).expect("no path given");    
    // 讀取檔案內容及錯誤處理
    if let Err(err) = get_csv_content(&path) {
        println!("error: {}", err);
        process::exit(1);
    }
}
