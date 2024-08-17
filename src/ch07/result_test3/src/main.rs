#![allow(unused)]

use reqwest;

async fn fetch_url(url: &str) -> Result<String, reqwest::Error>  {
    let content = reqwest::get(url).await.unwrap().text().await;
    // println!("{:?}", content);
    content
}

// read file
fn get_file_content(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(&path)?; // 可能發生錯誤
    Ok(content)
}

#[tokio::main]
async fn main() {
    // 讀取命令行參數, test: "https://www.rust-lang.org"
    // let path = "https://raw.githubusercontent.com/cs109/2014_data/master/countries.csv";
    let path = std::env::args().nth(1).expect("未指明檔案路徑或URL !!");
    
    if path.starts_with("http") {
        let _ = match fetch_url(&path).await {
            Err(error) => println!("{}", error),
            Ok(content) => println!("{:?}", content)
        };
    } else {
        // 讀取檔案內容及錯誤處理
        let _ = match get_file_content(&path) {
            Err(error) => println!("{}", error),
            Ok(content) => println!("{}", content)
        };
    }
}
