#![allow(dead_code)]

// Struct宣告
#[derive(Debug)]
struct Color1 {
    red: i32,
    blue: i32,
    green: i32,
}

#[derive(Debug)]
struct Color2(i32, i32, i32);

fn main() {
    let mut white = Color2(255, 255, 255);
    println!("{:?}", white);
    
    // 修改white
    white.0 = 254;
    println!("{:?}", white);
    
    // newtype：只有一個成員
    struct Inches(i32);
    let length = Inches(10);
    // 把 10 賦值給 integer_length
    let Inches(integer_length) = length; 
    println!("length is {} inches", integer_length);
}
