#![allow(unused)]

fn main() {
    // Closure
    let add = | a:i32, b:i32 | -> i32 { return a + b; };

    // 呼叫 Closure
    let x = add(1,2);
    println!("result is {}", x);
}
