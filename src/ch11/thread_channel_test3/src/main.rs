use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // 訊息
    let vals = vec![
        String::from("執行緒"),
        String::from("傳來"),
        String::from("的"),
        String::from("嗨"),
    ];
    
    thread::spawn(move || {
        // 分段傳送訊息
        for val in vals {
            tx.send(val).unwrap();
            // 傳送遲延，讓接收者不會一次收到所有訊息
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 分段接收訊息
    for received in rx {
        println!("取得：{}", received);
    }
}
