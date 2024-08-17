fn main() {
    // 簡單測試
    let bmi = 24.5;
    if bmi > 24.0 {
        println!("體重過重.");
    } else if bmi > 18.5 { 
        println!("體重適中.");
    } else {
        println!("體重過輕.");
    }
    
    // 結合let變數值指派
    let condition = true;
    let no = if condition { 5 } else { 6 };
    println!("no: {no}");
    
    // error
    // let no = if condition { 5 } else { "six" };
    
    // 四捨五入至小數點一位
    let bmi:f32 = format!("{:.1}", 30.0).parse().unwrap(); 
    // match，警告訊息：match不可使用浮點數，未來版本會視為錯誤
    match bmi {
        0.0..=18.5 => println!("體重過輕."),
        18.5..=24.0 => println!("體重適中."),
        24.0..=27.0 => println!("體重過重."),
        27.0..=30.0 => println!("輕度肥胖."),
        30.0..=35.0 => println!("輕度肥胖."),
        _ => println!("重度肥胖.")
    }

    // match 修正
    let bmi = 30; 
    match bmi {
        0..=18 => println!("體重過輕."),
        19..=24 => println!("體重適中."),
        25..=27 => println!("體重過重."),
        28..=30 => println!("輕度肥胖."),
        31..=35 => println!("輕度肥胖."),
        _ => println!("重度肥胖.")
    }
}
