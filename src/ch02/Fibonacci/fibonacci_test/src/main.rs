// 引用套件
extern crate num_bigint as bigint;
extern crate num_traits;
use bigint::BigUint;
use chrono::prelude::{DateTime, Utc};
use num_traits::{One, Zero};
// use std::io;

// 產生斐波那契數列
fn fibonacci(number: u128, acc: BigUint, current: BigUint) -> BigUint {
    if number == 0 {
        return acc;
    } else {
        return fibonacci(number - 1, &acc + current, acc);
    }
}

fn main() {
    // 產生斐波那契數列的個數
    let guess = 100;

    // 計算執行時間
    let old_time: DateTime<Utc> = Utc::now();

    println!("fibonacci number: {}", fibonacci(guess, Zero::zero(), One::one()));
    let duration = Utc::now().signed_duration_since(old_time);

    println!(
        "{} 毫秒(µs)",
        match duration.num_microseconds() {
            Some(value) => value,
            _ => 0,
        }
    );
}