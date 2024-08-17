macro_rules! create_function {
    // ident：表函數名稱或變數名稱
    ($func_name:ident) => {
        fn $func_name() {
            // stringify!：將 $func_name 轉換成字串
            println!("建立函數 {:?}", stringify!($func_name));
        }
    };
}

// 建立macro，顯示計算結果
macro_rules! print_result {
    ($x:expr) => {
        println!("{} = {:?}", stringify!($x), $x);
    };
}

fn main() {
    // 生成 macro
    create_function!(foo);
    create_function!(bar);
    
    // 測試函數
    foo();
    bar();

    // 測試函數
    print_result!(1u32 + 1);

    // Recall that blocks are expressions too!
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}
