#![allow(unused)]

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

fn main() {
    // 整數
    let x1 = Point { x: 5, y: 4 };
    println!("{:?}", x1);
    
    // 浮點數
    let x1 = Point { x: 5.0, y: 4.0 };
    println!("{:?}", x1);
    
    // 浮點數、整數混合
    let x1 = Point2 { x: 5.0, y: 4 };
    println!("{:?}", x1);
    
}
