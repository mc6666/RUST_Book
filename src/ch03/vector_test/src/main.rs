fn main() {
    // 建立 vector
    let mut numbers: Vec<i32> = Vec::new();

    // 推入3個元素
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    // 顯示 vector 內容及長度
    println!("Numbers: {:?}", numbers);
    println!("Length: {}", numbers.len());

    // 依據索引值讀取元素
    println!("Third number: {}", numbers[2]);

    // 依據索引值修改元素
    numbers[1] = 4;
    println!("Numbers: {:?}", numbers);

    // 使用迴圈讀取每一個元素
    for i in &numbers {
        println!("{}", i);
    }

    // 移出最上面(後面)的元素
    numbers.pop();
    println!("Numbers: {:?}", numbers);
}