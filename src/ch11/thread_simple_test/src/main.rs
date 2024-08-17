use std::thread;
use std::time::Duration;

fn main() {
    // 建立多執行緒
    for i in 1..11 {    // 10個執行緒
        thread::spawn(move || {
            println!("hi number {} from the spawned thread!", i);
            // 休眠 1 毫秒
            thread::sleep(Duration::from_millis(1));
            }
        );
    }

    // 5 個非執行緒休眠，各 1 毫秒
    for i in 1..6 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}