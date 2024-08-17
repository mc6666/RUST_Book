#![allow(unused)]

// Parent
trait Shape {
    fn draw(&self);
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
    let shape = ShapeFactory::new_shape(&ShapeType::Circle);
    shape.draw(); // output: draw a circle!

    // 創建矩形物件
    let shape = ShapeFactory::new_shape(&ShapeType::Rectangle);
    shape.draw(); // output: draw a rectangle!

    // 創建線段物件
    let shape = ShapeFactory::new_shape(&ShapeType::Line);
    shape.draw(); // output: draw a line!
}
