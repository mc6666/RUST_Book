// use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // let counter = Rc::new(Mutex::new(0));
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // let counter = Rc::clone(&counter);  // 複製指標
        let counter = Arc::clone(&counter);  // 複製指標
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // 存取 num 前，先鎖定
            *num += 1;
        });
        handles.push(handle);
    }

    // 保證所有執行緒會執行完畢，才結束主程式
    for handle in handles {
        handle.join().unwrap();
    }

    println!("結果：{}", *counter.lock().unwrap());
}
