use std::{error::Error, fs::File, process};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    city: String,
    region: String,
    country: String,
    population: Option<u64>,
}

// read csv
fn get_csv_content(path: &String) -> Result<(), 
                    Box<dyn Error>> {
    let file = File::open(&path)?;  // 開啟檔案
    let mut rdr = csv::Reader::from_reader(file); // 建立 CSV reader
    
    // header
    let headers = rdr.headers()?;
    println!("{:?}", headers);
    
    // data
    for result in rdr.records() {
        let record = result?;      // 檢查錯誤
        
        // 顯示一筆資料
        let city = &record[0];
        let region = &record[1];
        let country = &record[2];
        let pop: Option<u64> = record[3].parse().ok();

        println!( "city: {:?}, region: {:?},  country: {:?}, pop: {:?}",
                city, region, country, pop);
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
