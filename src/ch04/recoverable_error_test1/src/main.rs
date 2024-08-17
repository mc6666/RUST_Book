#![allow(unused)]

use std::fs::File;

fn age_from_input() -> Result<i32, String> {
    println!("輸入一個數字：");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let age: i32 = input.trim().parse().unwrap();
    Ok(age) 
}

fn main() {
    // 讀取檔案
    let path = "data.txt";
    let mut file = match File::open(&path) {
        Err(why) => panic!("無法開啟檔案：{}", &path),
        Ok(file) => file,
    };
    
    // 輸入一個數字
    let age = age_from_input();
    println!("{}", age.unwrap());
}
