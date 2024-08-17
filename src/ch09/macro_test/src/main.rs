#![allow(unused)]
use std::io::Write;

fn foo() {
    todo!("foo 尚未實作 !!");
}

fn os_type() {
    if cfg!(windows) {
        println!("Windows 作業系統.");
    } else {
        println!("Unix 作業系統.");
    };
}

fn main() {
    // 顯示至螢幕
    println!("Hello, world!");
    
    // format!
    let (x, y) = (1, 2);
    println!("format!: {}", format!("{x} + {y} = 3"));  
    
    // stringify!
    println!("stringify!: {}", stringify!(1 + 1));  
    
    // writeln!
    let mut w = Vec::new();
    writeln!(&mut w);
    writeln!(&mut w, "test");
    write!(&mut w, "formatted {}", "arguments");
    println!("write!: {:?}", w);  
    println!("write! to String: {}", String::from_utf8(w).unwrap());  
    
    // 顯示環境變數
    let path = env!("PATH");
    println!("env $PATH: {path}");
    
    // 顯示檔案名稱及行號
    println!("file: {}, line no.: {}", file!(), line!());
    
    // 顯示除錯訊息
    let a = 2;
    let b = dbg!(a * 2) + 1;
    
    os_type();
    
    foo();
}
