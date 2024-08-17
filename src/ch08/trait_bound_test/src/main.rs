#![allow(unused)]

// 使用struct定義屬性
#[derive(Debug, Clone, Copy)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

// 定義行為的規格
trait HasArea {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

// 以impl實作商業邏輯，計算圓的面積、周長
impl HasArea for Circle {
    // 面積
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
    
    // 周長
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

// 使用struct定義屬性
#[derive(Debug, Clone, Copy)]
struct Square {
    x: f64,
    y: f64,
    side: f64,
}

// 以impl實作商業邏輯，計算圓的面積、周長
impl HasArea for Square {
    // 面積
    fn area(&self) -> f64 {
        self.side * self.side
    }
    
    // 周長
    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }
}

// 泛型
fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}
fn print_perimeter<T: HasArea>(shape: T) {
    println!("This shape has an perimeter of {}", shape.perimeter());
}

// 測試：建立Circle物件，計算圓的面積
fn main() {
    let c = Circle {
        x: 0.0f64,
        y: 0.0f64,
        radius: 1.0f64,
    };

    let s = Square {
        x: 0.0f64,
        y: 0.0f64,
        side: 1.0f64,
    };

    print_area(c.clone());
    print_area(s.clone());
    // print_area(5);

    print_perimeter(c);
    print_perimeter(s);

}
