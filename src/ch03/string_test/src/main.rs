#![allow(unused)]
fn main() {
	// 宣告
    let x: &str = "hello";    
    println!("{x}");
    let x1 = "hello";    
    println!("{x1}");
    let x2: String = String::from("hello");
    println!("{x2}");
    let x3: String = "world".to_string();
    println!("{x3}");
    
    // 指標(pointer)使用的記憶體
    println!("傳統指標: {}", std::mem::size_of::<*const ()>());
    println!("&str: {}", std::mem::size_of::<&str>());
    
    // 轉換&str為String
    let s: &str = "Hello, world!";
    let string = s.to_string();

    // 轉換String為&str
    let string = String::from("Hello, world!");
    let s = string.as_str();
    let s2 = &string;

    // &str 切片(slicing)
	let substr : &str = &x[2..];  // 從第3個字元至最後
    println!("{substr}");
    let substr2 : &str = &x[..2];  // 從第1個字元至第2個字元
    println!("{substr2}");
    let substr3 : &str = &x[1..2];  // 取第2個字元
    println!("{substr3}");
    let substr4 : &str = &x[..];   // 取所有字元
    println!("{substr4}");
    
    // String 切片(slicing)
	let substr : &str = &x2[2..];  // 從第3個字元至最後
    println!("{substr}");
    let substr2 : &str = &x2[..2];  // 從第1個字元至第2個字元
    println!("{substr2}");
    let substr3 : &str = &x2[1..2];  // 取第2個字元
    println!("{substr3}");
    let substr4 : &str = &x2[..];   // 取所有字元
    println!("{substr4}");
    
    let c:Option<char> = x.chars().nth(2); // 取第3個字元
    println!("{c:?}");
    
    let c:Option<char> = x2.chars().nth(2); // 取第3個字元
    println!("{c:?}");
    
    // 字串連接(Concatenation)
    let concat1 = x2 + " " + x1 + " " + &x3;
    println!("x2 + x1 + &x3:{concat1}");
    
    // to_owned：複製 &str，且轉為 String
    let concat1 = x1.to_owned() + " " + x1;
    println!("x1 + x1:{concat1}");
    
    // 以下均不可行
    // let concat3 = x2 + " " + x2;
    // println!("x2 + x2:{concat3}");
    
    // let concat4 = x + x2;
    // println!("x + x2:{concat4}");
    
    // let concat5 = x + x;
    // println!("x + x:{concat5}");
    
    // String字串連接
    let mut s = String::from("Hello");
    s.push(' ');
    s.push_str("World.");
    println!("{}", s);
    
}
