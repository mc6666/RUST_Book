#![allow(unused)]

fn main() {
    // test 1
    let mut x1 = [1,2,3];
    let mut x2 = x1;  // 複製x1，x1所有權不會被轉移
    x2[0] = 10;       // x1 未隨之更改    
    println!("x1:{:?}", x1);  // x1可以再使用
    println!("x2:{:?}", x2);  

    // test 2
    let x1 = [1,2,3];
    let mut x2 = x1.to_vec();  // 複製x1，x1所有權不會被轉移
    x2.push(4);
    println!("x1.len：{}", x1.len());  // x1 可以再使用
    println!("x2.len：{}", x2.len());  

    // test 3
    let x1 = vec![1,2,3];
    let x2 = x1;  // x1 所有權會被轉移
    println!("{}", x1.len());  // x1 不可以再使用
}