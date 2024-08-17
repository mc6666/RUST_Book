// 讀取檔案
fn get_file_content(path: &String) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(&path)?; // 可能發生錯誤
    Ok(content)  // 成功就回傳檔案內容
}

fn main() {
    // 讀取命令行參數
    let path = std::env::args().nth(1).expect("未指明檔案路徑 !!");    
    // 讀取檔案內容及錯誤處理
    let _ = match get_file_content(&path) {
        Err(error) => println!("{}", error),
        Ok(content) => println!("{}", content)
    };
}
