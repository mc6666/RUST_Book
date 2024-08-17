#![allow(unused)]

use std::future::Future;
use futures::executor::block_on;

async fn borrow_x(x: &u8) -> u8 { *x + 1 }

fn test1() -> impl Future<Output = u8> {
    async {
        let x = 5;
        println!("{}", x);
        borrow_x(&x).await
    }
}

async fn test2(x: &mut i32) {
    *x = *x + 1;
    println!("{}", x);
}

fn main() {
    let x = test1();
    // println!("{}", x);

    block_on(test1());
    
    // no printing
    let mut x1 = 5;
    test2(&mut x1);
    
    // enable printing
    let mut x2 = 5;
    block_on(test2(&mut x2));
}
