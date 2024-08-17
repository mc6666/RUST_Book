#[no_mangle]
pub extern "C" fn rust_function() {
    println!("hello!");
}

#[no_mangle]
pub extern fn add_numbers(number1: i32, number2: i32) -> i32 {
    println!("Hello from rust!");
    number1 + number2
}