// 取索引值是4的元素
fn take_fifth(value: Vec<i32>) -> Result<i32, String> {
    if value.len() < 5 { 
        return Err("Index out of boundary !".to_string())
    } else {
        return Ok(value[4])
    }
}

fn main() {
    // 呼叫take_fifth函數取索引值是4的元素，陣列不足5個元素，會回傳錯誤
    let short_vec = vec![1, 2]; // 較短的陣列
    let result = take_fifth(short_vec);
    println!("{:?}", result.clone().ok());
    if result.is_err() {
        println!("{:?}", result.err())
    }

    // 回傳成功
    let long_vec = vec![1, 2, 3, 4, 5];  // 較長的陣列
    let result = take_fifth(long_vec);
    match result {
        Ok(v) => println!("value: {v:?}"),
        Err(e) => println!("錯誤訊息: {e:?}"),
    } 
}