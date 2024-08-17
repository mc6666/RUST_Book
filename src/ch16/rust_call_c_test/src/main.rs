#![allow(unused)]

use std::ffi::CString;
use std::os::raw::c_char;
// use libc::wchar_t;
use wchar::{wch, wchz, wchar_t};

extern "C" {
    fn abs(input: i32) -> i32;
    fn puts(s: *const c_char); // 顯示字串
    fn _putws(s: *const wchar_t); // 顯示中文字串
}

fn main() {
    // 設定C的字串型別
    let to_print = CString::new("Hello !").unwrap();
    
    unsafe {
        println!("呼叫C的絕對值(abs): {}", abs(-3));
        
        puts(to_print.as_ptr()); // 傳送字串指標
        
        let to_print: &[wchar_t] = wchz!("Rust中文測試");
        _putws(to_print.as_ptr()); // 傳送字串指標
    }
}