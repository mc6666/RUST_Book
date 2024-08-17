fn main() {
    // without move
    let mut num = 5;
    {
        let mut add_num = |x: i32| num += x;
        add_num(5);
    }
    println!("without move：{:#?}\n", num);

    // with move
    let mut num = 5;
    {
        let mut add_num = move |x: i32| num += x;
        add_num(5);
    }
    println!("with move：{:#?}\n", num);
    
    // Closure 可以回傳結果
    let mut counter = || {
        let mut count = 0;
        move || {
            count += 1;
            count // 回傳結果
        }
    }();
    println!("Counter: {}", counter()); // Output: Counter: 1
    println!("Counter: {}", counter()); // Output: Counter: 2
    
}
