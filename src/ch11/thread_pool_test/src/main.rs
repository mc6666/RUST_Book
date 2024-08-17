#![allow(unused)]

use std::thread;
use std::error::Error;
use reqwest;
use std::time::{Duration, Instant};
use std::io::Read;

fn get_data() -> Result<String, Box<dyn Error>> {
    let mut res = reqwest::blocking::get("http://localhost:8000")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    // println!("Status: {}", res.status());
    // println!("Headers:\n{:#?}", res.headers());
    // println!("Body:\n{}", body);

    Ok(body)
}

fn main() {
    let n_jobs = 10000;
    
    // 計時開始
    let start = Instant::now();

    // let barrier = Arc::new(Barrier::new(n_jobs + 1));
    let mut vec_handle = vec![];
    for _ in 0..n_jobs {
        vec_handle.push(thread::spawn(move || {
            let body = get_data();
        }));
    }
    // println!("Count:{}", vec_handle.len());
    
    // 保證所有執行緒會執行完畢，才結束主程式
    for handle in vec_handle {
        handle.join().unwrap();
    }
    
    let duration = start.elapsed();
    println!("耗時: {:?}", duration);

}