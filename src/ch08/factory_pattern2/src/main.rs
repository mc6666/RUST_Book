#![allow(unused)]

// Parent
trait Shape {
    fn draw(&self);
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn set_radius(&mut self, radius:i32);
}

enum ShapeType {
    Rectangle,
    Circle,
    Line
}

struct Rectangle {
    x:i32,
    y:i32,
    width:i32,
    height:i32
}

impl Shape for Rectangle {
    fn draw(&self) {
        println!("draw a rectangle!");
    }
    
    // 面積
    fn area(&self) -> f64 {
        (self.width * self.height) as f64
    }
    
    // 周長
    fn perimeter(&self) -> f64 {
        2.0 * self.width as f64 + 2.0 * self.height as f64
    }
    
    fn set_radius(&mut self, radius:i32) {
    }
}

struct Circle {
    x:i32,
    y:i32,
    radius:i32    
}

impl Shape for Circle {
    fn draw(&self) {
        println!("draw a circle!");
    }
    
    // 面積
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius as f64 * self.radius as f64)
    }
    
    // 周長
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius as f64
    }
    
    // 設定半徑
    fn set_radius(&mut self, radius:i32) {
        self.radius = radius;
    }
}

struct Line {
    x1:i32,
    y1:i32,
    x2:i32,
    y2:i32,
}

impl Shape for Line {
    fn draw(&self) {
        println!("draw a line!");
    }
    
    // 面積
    fn area(&self) -> f64 {
        0.0 as f64
    }
    
    // 周長
    fn perimeter(&self) -> f64 {
        let x_square:f64 = (self.x2 as f64-self.x1 as f64)*(self.x2 as f64-self.x1 as f64);
        let y_square:f64 = (self.y2 as f64-self.y1 as f64)*(self.y2 as f64-self.y1 as f64);
        (x_square+y_square).sqrt()
    }
    
    fn set_radius(&mut self, radius:i32) {
    }
}

struct ShapeFactory;
impl ShapeFactory {
    fn new_shape(s: &ShapeType) -> Box<dyn Shape> {
        match s {
            ShapeType::Circle => Box::new(Circle {x:0, y:0, radius:1}),
            ShapeType::Rectangle => Box::new(Rectangle {x:0, y:0, width:1, height:1}),
            ShapeType::Line => Box::new(Line {x1:0, y1:0, x2:1, y2:1}),
        }
    }
}

fn main() {
    // 創建圓形物件
    let mut shape = ShapeFactory::new_shape(&ShapeType::Circle);
    // shape.radius=5; // not work
    shape.draw(); // output: draw a circle!
    shape.set_radius(5);
    println!("{}", shape.area());

    // 創建矩形物件
    let shape = ShapeFactory::new_shape(&ShapeType::Rectangle);
    shape.draw(); // output: draw a rectangle!

    // 創建線段物件
    let shape = ShapeFactory::new_shape(&ShapeType::Line);
    println!("{}", shape.perimeter());
}
