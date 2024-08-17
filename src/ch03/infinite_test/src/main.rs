fn main() {
    // 顯示：無窮大、1/無窮大、無窮大/無窮大、1/0
    let inf = std::f32::INFINITY;
    println!("無窮大：{} {} {} {}", inf * 0.0, 1.0 / inf, inf / inf, 1.0 / 0.0);
    println!("無窮大檢查：{}", (1.0 / 0.0) == inf);
    
    // 遺失值(Missing value)
    let nan = std::f32::NAN;
    // 遺失值比較
    println!("遺失值：{} {} {}", nan < nan, nan > nan, nan == nan);
    println!("遺失值與0比較：{} {} {}", nan < 0.0, nan > 0.0, nan == 0.0);
    println!("遺失值檢查：{}", f32::is_nan(nan));
}
