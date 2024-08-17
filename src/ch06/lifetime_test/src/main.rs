// 根據 BMI 判斷體重 
// fn check_bmi(bmi:f32) -> &'static str {
fn check_bmi<'a>(bmi:f32) -> &'a str {
    if bmi > 24.0 {
        "體重過重."
    } else if bmi > 18.5 { 
        "體重適中."
    } else {
        "體重過輕."
    }
}

fn check_bmi2(bmi:f32) -> u8 {
    if bmi > 24.0 {
        0
    } else if bmi > 18.5 { 
        1
    } else {
        2
    }
}

fn main() {
    if std::env::args().len() <= 1 {
        println!("Usage：lifetime_test <bmi>");
        std::process::exit(1);
    }
    
    let bmi_str = std::env::args().nth(1).unwrap();
    let bmi:f32 = bmi_str.trim().parse().unwrap();
    println!("{}", check_bmi(bmi));
    println!("{}", check_bmi2(bmi));
}
