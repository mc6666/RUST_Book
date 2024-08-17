#![allow(unused)]
use std::ops::Mul;

struct Retangle<T> { // 矩形
    width: T,  // 寬
    height: T, // 高
}

// 方法
impl<T: Mul<Output = T>> Retangle<T> 
     where T: Copy + Mul<T, Output = T> {
    fn area(&self) -> T { // &self：物件本身
        self.width * self.height
    }
}

fn main() {
    // 顯示面積
    let x1 = Retangle { width: 5, height: 4 };
    println!("{}", x1.area());

    // 浮點數
    let x1 = Retangle { width: 5.0, height: 4.0 };
    println!("{}", x1.area());

}
