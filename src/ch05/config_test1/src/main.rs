use config::Config;
// use std::collections::HashMap;
use rand::Rng;
use std::io;

fn main() {
    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("./Settings"))
        .build()
        .unwrap();
            
    // 一次讀取所有的組態變數
    // settings.clone() // clone() 是避免 method 2 無所有權
        // .try_deserialize::<HashMap<String, String>>()
        // .unwrap();
        
    // 顯示所有組態變數
    println!("{:#?}", settings);
    
    // 讀取單一變數 
    let max_count:i64 = settings.get_int("max_count").unwrap();
    
    // 隨機產生一個數字，介於[0, 9]
    let mut rng = rand::thread_rng();
    let answer:u8 = rng.gen_range(0..10);
    
    let mut count = 0; // 猜錯的次數初始值=0
    loop {
        let mut guess_no = String::new();
        println!("猜一個數字(0~9)：");
        io::stdin().read_line(&mut guess_no)
            .expect("Failed to read line"); // 例外控制
            
        // 字串轉數字
        let guess_no:u8 = guess_no
            .trim()  // 去除首尾空白
            .parse() // 解析，將字串轉數值
            .expect("輸入不是數字."); // 例外控制
            
        if guess_no != answer {
            println!("猜錯了.");
            count += 1;
        } else {
            println!("猜對了.");
            break;
        }                     

        if count >= max_count {
            println!("失敗. 答案是{answer}");
            break;
        }
    }
}
