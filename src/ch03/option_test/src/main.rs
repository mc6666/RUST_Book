#![allow(dead_code)]

// 取索引值是4的元素
fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 { 
        None
    } else {
        Some(value[4])
    }
}

fn main() {
    // 呼叫take_fifth函數取索引值是4的元素，陣列不足5個元素，會回傳None
    let short_vec = vec![1, 2]; // 較短的陣列
    let long_vec = vec![1, 2, 3, 4, 5];  // 較長的陣列
    println!("{:?}, {:?}", take_fifth(short_vec), take_fifth(long_vec));
    
    // 使用is_some()檢查是否有值，is_none()檢查是否為None
    let short_vec = vec![1, 2]; // 較短的陣列
    let long_vec = vec![1, 2, 3, 4, 5];  // 較長的陣列
    let (x1, x2) = (take_fifth(short_vec), take_fifth(long_vec));
    println!("is_some：{:?}, {:?}", x1.is_some(), x2.is_some());
    println!("is_none：{:?}, {:?}", x1.is_none(), x2.is_none());
    
    // 將 Option<i32> 轉換為 i32
    let mut x:i32=0;
    match x1 {
      Some(number) => x = Some(number+1).unwrap(),
      None => println!("Found a None!"),
    }
    println!("x1：{:?}", x);

    // let mut x:Option<i32>=Some(0);
    // match x2 {
      // Some(number) => x = Some(number+1),
      // None => println!("Found a None!"),
    // }
    x = x2.unwrap();
    println!("x2：{:?}", x);
 
}