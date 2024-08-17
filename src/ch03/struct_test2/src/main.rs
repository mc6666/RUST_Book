// Rust By Example, 9.1 Methods

// 點
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

// 方法
impl Point {
    // 原點
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // 給定初始值
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

// 矩形
#[derive(Debug)]
struct Rectangle {
    p1: Point, // 左上角座標
    p2: Point, // 右下角座標
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // 面積
        ((x1 - x2) * (y1 - y2)).abs()
    }

    // 週長
    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // 平移
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

fn main() {
    // 新增一個矩形物件
    let mut rectangle = Rectangle {
        // Associated functions are called using double colons
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    
    // 顯示週長、面積
    println!("週長: {}", rectangle.perimeter());
    println!("面積: {}", rectangle.area());

    // 平移
    rectangle.translate(1.0, 1.0);
    println!("{:?}", rectangle);
}
