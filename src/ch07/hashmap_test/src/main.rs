use std::collections::HashMap;

fn main() {
    // 初始化一個空的HashMap
    let mut fruits: HashMap<i32, String> = HashMap::new();
    
    // 新增元素
    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("Orange"));
    fruits.insert(3, String::from("Grape"));
    fruits.insert(4, String::from("Banana"));
    
    println!("{:?}", fruits);
    
    // 查詢
    println!("{}", fruits.get(&3).unwrap());
    
    // 更正
    fruits.insert(2, String::from("Mango"));
    println!("{}", fruits.get(&2).unwrap());
    
    // 刪除
    fruits.remove(&3);
    println!("{:?}", fruits);
    println!("{:?}", fruits.get(&3));
    
    // 其他方法
    println!("len：{:?}", fruits.len()); // 元素個數
    println!("contains_key：{:?}", fruits.contains_key(&5)); // 是否包含key值
    
    // 顯示所有key、value
    for (key, value) in fruits.iter() {
        println!("{}:{}", key, value);
    }
    
    // 顯示所有key、value
    for key in fruits.keys() {
        println!("{}", key);
    }
    
    // 顯示所有key、value
    for value in fruits.values() {
        println!("{}", value);
    }
    
    // 以key排序
    let mut hash_vec: Vec<_> = fruits.iter().collect();
    println!("\n以key排序：");
    hash_vec.sort_by(|a, b| a.0.cmp(b.0));

    for item in hash_vec {
        println!("{:?}", item);
    }
    
    // 以key降冪排序
    let mut hash_vec: Vec<_> = fruits.iter().collect();
    println!("\n以key降冪排序：");
    hash_vec.sort_by(|a, b| b.0.cmp(a.0));

    for item in hash_vec {
        println!("{:?}", item);
    }
    
    // 以value排序
    let mut hash_vec: Vec<_> = fruits.iter().collect();
    hash_vec.sort_by(|a, b| a.1.cmp(b.1));

    println!("\n以value排序：");
    for item in hash_vec {
        println!("{:?}", item);
    }
    
}
