fn main() {
    let x = [1, 2, 3, 4, 5]; // 自動偵測資料型態
    println!("{x:?}");
    let x:[i32; 5] = [1, 2, 3, 4, 5];
    println!("{x:?}");
    
    let x2 = [3; 5];  // 重複5個3
    println!("{x2:?}");
    
    // 存取單一元素
    let first = x[0];
    let second = x[1];
    println!("{first}, {second}");
    
    // 修改元素
    let mut x = [1, 2, 3, 4, 5];
    let second = x[1];
    x[1] = 100;
    println!("{x:?}");
    println!("second:{second}");
    
    // 存取部分範圍元素
    let x = [1, 2, 3, 4, 5];
    let a = &x[1..3];
    println!("{a:?}");
    
    // 修改部分範圍元素
    let x = [1, 2, 3, 4, 5];
    let mut b = &x[1..3];
    b=&[100];
    println!("修改部分範圍元素：{x:?}, {b:?}");
    
    // 陣列連接
    let mut x = [1, 2, 3, 4];
    let y = [5, 6, 7, 8]; 
    let concatenated = x.concat(&y);
    println!("{x:?}");
}
