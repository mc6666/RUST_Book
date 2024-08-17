#![allow(unused)]

fn main() {
    // 數值資料型別轉換
    let x1: i32 = 10;
    let x2: f32 = 20.5;
    let result = x1 + x2 as i32;
    println!("{}", result);

    // 字串轉換為數值
    let x1: String = "10".to_string();
    let x2: f32 = 20.5;
    let result = x1.trim().parse::<f32>().unwrap() + x2;
    println!("{}", result);

    // 數值轉換為字串
    let x1: f32 = 20.5;
    let result = x1.to_string() + "元";
    println!("{}", result);

    // char轉換為數值
    let x1 = '2';
    let result = x1.to_digit(10).unwrap(); // 10 進位
    println!("{}", result);
    let x1 = 'f'; 
    let result = x1.to_digit(16).unwrap(); // 16 進位
    println!("{}", result);

    // 字串轉換為bytes
    let x1 = "中文";
    println!("{:?}", x1.as_bytes());

    // bytes轉換為字串
    let x2 = String::from_utf8(x1.as_bytes().to_vec()).unwrap();
    println!("{}", x2);

    // 取得資料型別
    let x1: f32 = 20.5;
    println!("{:?}", print_type_of(&x1));
    
    let x1 = "中文";
    println!("{:?}", print_type_of(&x1));
    
    let x1:String = "中文".to_string();
    println!("{:?}", print_type_of(&x1));
    
    let x1:String = "中文".to_string();
    println!("{:?}", print_type_of(&x1.as_bytes()));

}


fn print_type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}