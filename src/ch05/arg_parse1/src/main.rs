use std::env;

fn main() {
    // 將所有參數存入 args 陣列
    let args: Vec<String> = env::args().collect();
    
    // 顯示所有參數值
    for i in 0..args.len() {
        println!("{}", &args[i]);
    }
}
