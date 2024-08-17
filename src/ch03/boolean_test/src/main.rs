fn main() {
    let x1 = true;
    println!("{x1}");
    
    let x2 = !x1; // 否定
    println!("否定：{x2}");
    
    // and 
    println!("and：{}", x1 && x2);

    // or 
    println!("or：{}", x1 || x2); 
    
    // 比較
    let (x, y) = (1, 2); 
    println!("x == y：{}", x == y);
}
