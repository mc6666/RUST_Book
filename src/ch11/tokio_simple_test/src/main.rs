#![allow(unused)]

async fn test2(x: &mut i32) {
    *x = *x + 1;
    println!("{}", x);
}

#[tokio::main]
async fn main() {
    let mut x1 = 5;
    test2(&mut x1);
    println!("程式結束 !!");
}