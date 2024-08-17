#![allow(unused)]

fn simpl_call<F>(closure1: F) -> i32
    where F: Fn(i32) -> i32 {

    closure1(1)
}

fn call_add<F>(closure1: F) -> i32
    where F: Fn(i32, i32) -> i32 {

    closure1(2, 3)
}

fn call_function<F>(closure1: F, a:i32, b:i32) -> i32
    where F: Fn(i32, i32) -> i32 {

    closure1(a, b)
}

fn main() {
    // 呼叫 Closure
    let result = simpl_call(|x| x + 2);
    println!("result is {}", result);
    
    let result = simpl_call(|x| x + 5);
    println!("result is {}", result);
    
    // Closure
    let add = | a:i32, b:i32 | -> i32 { return a + b; };

    // 呼叫 call_add
    let result = call_add(add);
    println!("result is {}", result);
    
    // 呼叫 call_function
    let result = call_function(add, 5, 10);
    println!("result is {}", result);
    
    // 使用乘法，呼叫 call_function
    let result = call_function(|a, b| a * b, 5, 10); // 簡略的寫法
    println!("result is {}", result);
}
