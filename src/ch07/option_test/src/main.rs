fn find_element(value: Vec<&str>, index:usize) -> Option<&str> {
    if index >= value.len() { 
        None
    } else {
        Some(value[index])
    }
}

fn main() {
    // 讀取命令行參數
    let index = std::env::args().nth(1).expect("未指定索引值.");    
    let index = index.trim().parse().expect("參數須為數值.");    
    // 根據索引值，找出水果名稱
    let vec = vec!["Apple", "Orange", "grape", "Strawberry"];
    let x: Option<&str> = find_element(vec, index);
    // 顯示結果
    if x == None {
        println!("索引值超出範圍.");
    } else {
        println!("{}", x.unwrap());
    }
}
