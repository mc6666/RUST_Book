#![allow(unused)]

// 使用struct定義屬性
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

// 定義行為的規格
trait HasArea {
    fn area(&self) -> f64;
}

// 以impl實作商業邏輯，計算圓的面積
impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

// 測試：建立Circle物件，計算圓的面積
fn main() {
    let circle = Circle{x:5.0, y:10.0, radius:5.0};
    println!("{}", circle.area());
}
