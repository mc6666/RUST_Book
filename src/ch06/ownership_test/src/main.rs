#![allow(unused_variables)]

fn main() {
    // example 1
    let x2:&str;
    {
        let x1 = "hello";
    }
    println!("{x1}");

    // example 2
    let x2:&str;
    {
        let x1 = "hello";
        x2 = x1;
    }
    println!("{x2}");

    // example 3，基本型別指派或傳遞都是複製，沒有所有權轉移的問題
    let x1:i32 = "2".trim().parse().unwrap();
    println!("{}", x1.rotate_left(1)); // 顯示 x1 位元左移

    let x2 = x1.to_string();  // x1 所有權不會被轉移(move)
    println!("{}", x1.rotate_left(1)); // x1 可以再使用 
    
    // example 4
    let x1:String = "hello".to_string();
    println!("{}", x1.len()); // 顯示 x 字串長度

    let x2 = x1.into_bytes();  // x1 所有權會被轉移(move)
    println!("{}", x1.len()); // x1 不可以再使用 
    
    // example 5
    let x1:String = "hello".to_string();
    println!("{}", x1.len()); // 顯示 x 字串長度

    let x2 = x1.clone().into_bytes();  // x1 所有權不會被轉移
    println!("{}", x1.len()); // x1 可以再使用 

    
    // example 6, 使用位址可以避免所有權會被轉移
    let x1 = gives_ownership();
    println!("{}", x1); 
    let x1 = takes_and_gives_back(x1);
    println!("{}", x1); 

    let x:&str ="hello";
    println!("{}", x); 
    let ptr = *&x;
    println!("{}", ptr); 
    println!("{}", x); 
}

fn gives_ownership() -> String {
    let some_string = "hello".to_string();
    some_string                              
}

fn takes_and_gives_back(a_string: String) -> String { 
    a_string + " too"
}
