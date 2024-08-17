#![allow(unused)]

fn main() {
    // test data
    let a = ["a", "b", "c"];
    let b = [1, 2, 3];

    // zip test
    let arr: Vec<_> = a.into_iter().zip(b.into_iter()).collect();
    println!("{:?}", arr);
}
