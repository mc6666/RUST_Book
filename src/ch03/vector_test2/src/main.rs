use std::mem::size_of_val;

fn main() {
    // 建立 vector
    let mut names = vec!["Alice", "Bob", "Charlie"];

    // 在索引值0之前插入元素
    names.insert(0, "Eve");
    println!("Names: {:?}", names);

    // 依據索引值刪除元素
    names.remove(1);
    println!("Names: {:?}", names);

    // 升冪排序
    names.sort();
    println!("Names: {:?}", names);

    // 降冪排序
    names.sort_by(|a, b| b.cmp(a));
    println!("Names: {:?}", names);

    // 依函數排序
    let mut arr = vec!["Alice", "Bob", "Charlie"];
    arr.sort_by_key(|s| s.len());
    println!("sort_by_key: {:?}", arr);
    
    let mut arr = [-2i32, -1, 0, 3];
    arr.sort_by_key(|n| n.abs());
    println!("sort_by_key: {:?}", arr);

    // 反轉
    names.reverse();
    println!("Names: {:?}", names);

    // 額外保留空間
    names.reserve(10);
    println!("Capacity: {}", names.capacity());
    println!("Length: {}", names.len());
    println!("Size: {}", size_of_val(&names));
}