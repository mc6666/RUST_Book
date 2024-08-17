fn main() {
    // unwrap_or
    let arg1 = std::env::args().nth(1).unwrap_or("".to_string()); // 取得第一個參數
    let arg2 = std::env::args().nth(2).unwrap_or("0".to_string()); // 取得第二個參數
    println!("{} {}", arg1, arg2);
    
    let n: i32 = arg1.trim().parse().unwrap_or(0); // 預設值為0
    println!("{}", n);
    
    // unwrap_or_else
    // let arg1: Result<i32, &str> = Err("error message");
    let n: i32 = arg2.trim().parse().unwrap_or_else(|x| {
        println!("error message: {}", x);
        0
    });
    println!("{}", n);

    // Closure + Result
    let arg3: Result<i32, &str> = Err("重大錯誤.");
    let _: i32 = arg3.unwrap_or_else(|x| {
        println!("error message: {}", x);
        0
    });

}