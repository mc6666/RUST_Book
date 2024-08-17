fn main() {
    let x1 : i32 = 32;     // 十進位表示
    let x2 : i32 = 0xFF;   //  0x：16進位
    let x3 : i32 = 0o55;   //  0o：8進制
    let x4 : i32 = 0b1001;  //  0b：2進位
    let x5 : i32 = 1_000_000;
    println!("{x5}");
    
    // isize最大值測試
    println!("isize最大值={}", isize::MAX);
    let base : i128 = 2;
    let exp : i128 = base.pow(63);
    println!("2^(64-1)=   {exp}");
    
    // usize最大值測試
    println!("usize最大值={}", usize::MAX);
    let base : u128 = 2;
    let exp : u128 = base.pow(64);
    println!("2^64=	    {exp}");
}
