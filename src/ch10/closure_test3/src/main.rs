#![allow(unused)]

#[derive(PartialEq, Debug, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // 陣列高階函數 map
    let multiple2 = | a | a * 2;
    let val = Some(2);
    let x: Option<i32> = val.map(multiple2);
    println!("{}", x.unwrap());

    // None 測試
    let val:Option<i32> = None;
    let x: Option<i32> = val.map(multiple2);
    println!("{:?}", x);
    
    // 陣列高階函數 sort_by_key
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // 依照寬度(width)排序
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("排序：\n{:#?}\n, sorted in {num_sort_operations} operations", list);

    // 依照寬度(width)過濾
    let mut list2 = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    let result = list2.into_iter().filter(|r| {
        r.width > 5
    }).collect::<Vec<_>>();
    println!("過濾：\n{:#?}\n", result);
    
    // 依照單字長度排序
    let mut fruits = vec!["banana", "apple", "orange", "grape"];
    fruits.sort_by(|a, b| a.len().cmp(&b.len()));
    println!("Sorted fruits: {:?}", fruits); 
}
