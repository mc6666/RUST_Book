use std::sync::mpsc;
use std::thread;

fn main() {
    // 建立通道
    let (tx, rx) = mpsc::channel();

    // 建立執行緒
    thread::spawn(move || {
        let val = String::from("嗨");
        // 傳送訊息
        tx.send(val).unwrap();
    });

    // 接收訊息
    let received = rx.recv().unwrap();
    println!("取得：{}", received);
}
