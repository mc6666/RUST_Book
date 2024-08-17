#![allow(unused)]

use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list1 = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("{:?}", list1);
    
    let list2 = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list2);
}
