use std::io;

fn main() {
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    println!("{months:?}");
    
    println!("輸入索引值:1~{}", months.len());

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index) // 讓使用者輸入索引值:
        .expect("Failed to read line");

    let index: usize = index
        .trim()  // 去除首尾空白
        .parse() // 解析，將字串轉數值
        .expect("輸入不是數字."); // 例外控制

    let element = months[index-1];
    println!("索引 {index}: {element}");    
}
