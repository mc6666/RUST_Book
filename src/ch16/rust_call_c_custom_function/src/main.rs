extern crate core;
use core::ffi::c_int;

// 宣告C的multiply函數規格
extern "C" {
    fn multiply(a: c_int, b: c_int) -> c_int;
}

fn main() {
    println!("[Rust] Hello from Rust!");

    unsafe {
        println!("[Rust] Calling function in C..");
        
        // 呼叫C的multiply函數
        let result = multiply(5000, 5);

        println!("[Rust] Result: {}", result);
    }
}
