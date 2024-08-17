use walkdir::WalkDir;
use std::thread;

fn find_pattern(path:&String, pattern:&String) -> Result<String, std::io::Error> {
    let mut s = String::from("");

    // check file exist?
    if !std::path::Path::new(&path).exists() {
        panic!("檔案 {path} 不存在.");
    }
        
    let content = std::fs::read_to_string(&path)?;
    
    // 逐行比對
    for (line_number, line) in content.lines().enumerate() {
        if line.contains(pattern) {
            // println!("{}: {}", line_number, line);
            s.push_str(&format!("{}: {}\n", line_number, line));
        }
    }
    Ok(s)
}

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    
    // check file exist?
    if !std::path::Path::new(&path).exists() {
        panic!("檔案 {path} 不存在.");
    }

    if std::path::Path::new(&path).is_file() {
        // 逐行比對
        let _ = match find_pattern(&path, &pattern){
            Err(error) => panic!("{error}"),
            Ok(contents) => println!("{}", contents)
        };
    } else {
        for e in WalkDir::new(path).into_iter().filter_map(|e| e.ok())
            .filter(|e| match e.path().extension() {
                None => false,
                Some(ex) => ex == "rs"})
        {
            if e.metadata().unwrap().is_file() {
                let pattern_tmp = pattern.clone(); // 複製，避免所有權被借用
                // 產生一個新的執行緒
                let handle = thread::spawn(move || { // move：所有權轉移
                    let file_name = format!("{}", e.path().display());
                    
                    let _ = match find_pattern(&file_name, &pattern_tmp){
                        Err(error) => panic!("{error}"),
                        Ok(contents) => if contents.len() > 0 {
                            println!("{file_name}:\n{}", contents);
                            } 
                    };
                });
                handle.join().unwrap(); //要求主程式等執行緒完成工作才能結束程式
            }
        }
    }
}
