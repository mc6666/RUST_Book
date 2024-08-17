fn main() {
    // 基本測試
    let f1 : f64 = 2.0; // 必須加 .0
    println!("{f1}");
    let f1_1 : f64 = 2 as f64; // 轉型
    println!("{f1_1}");
    let f2 : f64 = 1_000_000.0; 
    println!("{f2:0e}");
    
    // 最大值測試
    println!("f32最大值={}", f32::MAX);
    println!("f64最大值={}", f64::MAX);

    // 浮點數運算
    let a = 10.0;
    let b = 10.0;
    let c = a + b;
    println!("a+b={c}");
}
