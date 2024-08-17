#![allow(unused)]
// use std::io::Write;
use std::fmt::Debug;

trait ApproxEqual {
    fn approx_equal(&self, other: &Self) -> bool;
}
impl ApproxEqual for f32 {
    fn approx_equal(&self, other: &Self) -> bool { // 大約等似
        (self - other).abs() <= ::std::f32::EPSILON
    }
}

fn foo<T: Clone + Debug>(x: T) {
    x.clone();
    println!("{:?}", x);
}

fn main() {
    println!("ε：{}", std::f32::EPSILON);
    println!("大約等於：{}", (1.0).approx_equal(&1.00000001));
    
    // 寫入檔案
    let mut f = std::fs::File::create("foo.txt").expect("Couldn’t create foo.txt");
    let buf = b"whatever"; // buf: &[u8; 8], a byte string literal.
    let result = f.write(buf);

    // Clone + Debug
    let x = "hello";
    foo(x);
}
