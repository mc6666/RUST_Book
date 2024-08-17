fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 解構(destructuring)
    let (x, y, z) = tup; // x = 500, y = 6.4, z = 1
    println!("The value of y is: {y}");
    
    let five_hundred = tup.0;
    println!("The value of five_hundred is: {five_hundred}");
}