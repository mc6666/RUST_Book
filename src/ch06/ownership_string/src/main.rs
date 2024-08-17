#![allow(unused)]

fn main() {
    // test 1
    // String does not implement the `Copy` trait
    let x1:String = "hello".to_string();
    println!("{}", x1.len()); // 顯示 x1 字串長度

    // let x2 = &x1.into_bytes();  // x1 所有權會被轉移(move)
    let x2 = &x1.clone().into_bytes();  // x2 複製 x1，x1 可以再使用
    println!("{}", x1.len()); // 顯示 x1 字串長度
    
    // test 2
    // &str is OK
    let x1:&str = "hello";
    println!("{}", x1.len()); // 顯示 x1 字串長度

    let x2 = &x1.bytes();  
    println!("{}", x1.len());  // x1 可以再使用
    
}
