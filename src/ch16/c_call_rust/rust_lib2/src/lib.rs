#[no_mangle]
pub extern "C" fn rust_function() -> i32 {
    println!("hello!");
    return 5;
}