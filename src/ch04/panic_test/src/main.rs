use std::io;

fn main() {
    let x = 1;
    let x2 = 5;
    let y = x / (x2-5*x);
    println!("{y}");
    
    let mut no = String::new();
    println!("輸入一個數字：");
    io::stdin().read_line(&mut no)
        .expect("Failed to read line"); // 例外控制
        
    // 字串轉數字
    let no:u8 = no
        .trim()  // 去除首尾空白
        .parse() // 解析，將字串轉數值
        .expect("輸入不是數字."); // 例外控制
    
    let y = x / no;
    println!("{y}");
}