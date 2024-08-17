use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    // move 確保變數tx會轉移所有權
    thread::spawn(move || {
        let val = String::from("嗨");
        // tx.send會轉移變數val所有權
        tx.send(val).unwrap();
        // tx.send(val.clone()).unwrap();
        // 發生錯誤，val所有權已被轉移
        println!("val 為 {}", val);
    });

    let received = rx.recv().unwrap();
    println!("取得：{}", received);
}
