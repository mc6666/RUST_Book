#![allow(unused)]

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

/* fn rc_test() {
    let s = Rc::new(String::from("test"));
    for _ in 0..10 {
        let s = Rc::clone(&s);
        let handle = thread::spawn(move || {
            println!("{}", s)
        });
        handle.join().unwrap();
    }
}
 */
fn arc_test() {
    let s = Arc::new(String::from("test"));
    for _ in 0..10 {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || {
           println!("{}", s)
        });
        handle.join().unwrap();
    }
}

fn arc_test2() {
    let counter = Arc::new(Mutex::new(0)); // 宣告一個智慧指標Arc
    for _ in 0..10 {
        let counter_arc = Arc::clone(&counter); // 複製智慧指標副本
        let handle = thread::spawn(move || {
           *counter_arc.lock().unwrap() +=1;  // 鎖定並更新變數值
        });
        handle.join().unwrap();
    }
   println!("{:?}", counter);
}

fn main() {
    // rc_test();
    // arc_test();
    arc_test2();
}