fn main() {
    // 不同資料類型的變數組合在一起，不須轉型
    let name = "Michael";
    let age = 25;
    let formatted_string = format!("我是 {name}，今年 {age} 歲.");
    println!("{}", formatted_string);
    
    // 四捨五入
    let pi = 3.14159;
    let formatted_string = format!("{pi:.3}");
    println!("pi:{}", formatted_string);
    
    // 不同字串資料類型的變數組合在一起，不須轉型
    let x1: &str = "hello1";    
    let x2: String = String::from("hello2");
    let x3: String = "hello3".to_string();
    let formatted_string = format!("{x1}\n{x2}\n{x3}\n");
    println!("字串連接:\n{}", formatted_string);

    // 指定參數對應順序
    let day = 2;
    let month = "January";
    let year = 2023;
    let formatted_string = format!("{}-{}-{}", year, month, day);
    println!("{}", formatted_string);
    let formatted_string = format!("{1}. {2} {0}", year, &month[..3], day);
    println!("{}", formatted_string);
    let formatted_string = format!("{1:.3}. {2} {0}", year, month, day);
    println!("{}", formatted_string);
    
    // .*
    println!("{}, `{name:.*}` 有 3 位小數", "Hello", 3, name=1234.56);
    println!("{}, `{name:.*}` 有 3 個字元", "Hello", 3, name="1234.56");
    println!("{}, `{name:>8.*}` 有 3 個靠右字元", "Hello", 3, name="1234.56");
}
