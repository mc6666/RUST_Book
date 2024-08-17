// 找最大值：同時支援整數及字元陣列
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    // 整數陣列測試
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    // 字元陣列測試
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // 字串陣列測試
    let string_list = vec!["A1", "A3", "A2", "B"];
    let result = largest(&string_list);
    println!("The largest string is {}", result);
}
