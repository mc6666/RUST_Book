#![allow(unused)]

fn main() {
    // 使用map處理單一數值
    let multiple2 = | a | a * 2; // 將變數值乘以2
    let val = Some(2);
    let x: Option<i32> = val.map(multiple2);
    println!("{}", x.unwrap());

    // 使用map處理陣列
    let capitalize = |value: &str| value.to_uppercase();  // 將變數值轉換為大寫
    let cities = vec!["rome", "barcelona", "berlin"];
    let cities_caps: Vec<String> = cities.clone().into_iter().map(capitalize).collect();
    println!("{:?}", cities_caps);
    
    // for_each
    let capitalize_for = |value: &str| println!("{:?}", value.to_uppercase());  // 將變數值轉換為大寫
    cities.clone().into_iter().for_each(capitalize_for);
    
    // for_each with vec
    let mut vec = Vec::new();
    let capitalize_for = |value: &str| vec.push(value.to_uppercase());  // 將變數值轉換為大寫
    cities.into_iter().for_each(capitalize_for);
    println!("{:?}", vec);
    
    // unwrap_or_else with vec
    let vec = vec![8, 9, 10];
    let fourth = vec.get(3).unwrap_or_else(|| &0);
    println!("fourth element：{}", fourth);

    // map + take_while + filter
    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }
    
    let combined_closure: u32 =
        (0..6).map(|n| n * n)         // 將變數值平方
             .take_while(|&n| n < 50) // 篩選 < 50 的值
             .filter(|&n| is_odd(n))  // 篩選單數
             // .map(|n| n * n)
             .sum();                  // 加總
    println!("v1. map + take_while + filter：{}", combined_closure);
    
    // 將 Closure 指派給變數，以利重複使用
    let exp = |n: u32| n * n; // 將變數值平方
    let upper_limit = |n: &u32| n < &50; // 篩選 < 50 的值
    let is_odd = |n: &u32| n % 2 == 1;  // 判斷是否為單數
    let combined_closure: u32 =
        (0..6).map(exp)              // 平方
             .take_while(upper_limit) // 不可大於或等於上限
             .filter(is_odd)     // 篩選單數
             .sum();  // 加總
    println!("v2. map + take_while + filter：{}", combined_closure);   
    
    // 重複使用
    let combined_closure2: u32 =
        (0..10).map(exp)              // 平方
             .take_while(upper_limit) // 不可大於或等於上限
             .filter(is_odd)     // 篩選單數
             .sum();  // 加總
    println!("v3. map + take_while + filter：{}", combined_closure2);   
    
    // count：計算筆數
    let combined_closure2: u32 =
        (0..10).filter(is_odd)     // 篩選單數
             .count().try_into().unwrap();  // 筆數
    println!("count：{}", combined_closure2);   
    
    // filter_map：filter + map
    let vec = vec!["8", "9", "ten", "11", "twelve"];
    let filter_vec = vec
        .into_iter()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect::<Vec<i32>>();
    println!("{:?}", filter_vec);
    
    // fold：類似Python的Reduce
    let vec = vec![1, 2, 3, 4, 5];
    let filter_vec = vec.iter()
        .filter(|&x| x % 2 == 1)    // 篩選單數
        .fold(0, |acc, x| acc + x); // 累加
    println!("{:?}", filter_vec);
    
}
