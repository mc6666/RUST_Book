#![allow(unused)]

fn main() {
    // 位址及指標操作
    let x: i32 = 10; // x為一整數
    let x_address = &x; // x_address 是 x 的記憶體位址
	let x_pointer: *const i32 = &x; // x_pointer 儲存 x_address
    println!("x: {}", x);
    println!("x_address:{}", x_address);
    println!("x_pointer:{:?}", x_pointer);

    // 顯示位址
    println!("\n顯示位址：");
    println!("&x: {:p}", &x);
    println!("x_address:{:p}", x_address);
    println!("&x_pointer:{:p}", &x_pointer);

    // 解除參考
    unsafe {
        println!("*x_pointer:{}", *x_pointer);
    }
    
    // 變更指標所指的內容
    let mut x = 5;
    let y = &mut x;
    *y = 6;
    println!("x={x}");
    
    // 利用指標修改資料內容
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // 每一個元素各加50
    }
    println!("\n利用指標修改資料內容：{:?}", v);
    
    
    // 字串指標測試
    let x: &str = "hello";  // 宣告變數x
	let x_pointer: *const str = x; // x_pointer 儲存 x_address
    println!("\n字串指標測試：");
    println!("x: {}", x);
    println!("&x: {}", &x);
    println!("x_pointer:{:?}", x_pointer);
}
