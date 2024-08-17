
// 動態呼叫 Closure
type DynamicClosure = Box<dyn Fn(i32, i32) -> i32>;
fn closure1() -> DynamicClosure {
    // 回傳 Closure
    Box::new(| a:i32, b:i32 | -> i32 { return a + b; })
}

fn closure2() -> DynamicClosure {
    // 回傳 Closure
    Box::new(| a:i32, b:i32 | -> i32 { return a - b; })
}

fn execute_closure(f: DynamicClosure, arg1: i32, arg2: i32) -> i32 {
    f(arg1, arg2)
}

fn main() {
    // 呼叫 Closure
    let mut action = "add"; // 指令
    let mut func;
    match action { // 依據指令動態決定要取得哪一個函數
        "add" => func = closure1(),
        _ => func = closure2(),
    };
    let x = execute_closure(func, 30, 20);  // 動態呼叫函數
    println!("result is {}", x);
    
    // 呼叫 Closure
    action = "minus"; // 指令
    match action { // 依據指令動態決定要取得哪一個函數
        "add" => func = closure1(),
        _ => func = closure2(),
    };
    let x = execute_closure(func, 30, 20); // 動態呼叫函數
    println!("result is {}", x);
}
