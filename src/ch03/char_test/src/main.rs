fn main() {
    let love:char = '❤';
    let chinese:char = '中';
    let special:char = '\n'; // 換行
    let unicode1:char = '\x07'; // 16進位, beep
    let unicode2:char = '\u{03B5}'; // unicode ε
    let x1:u8 = b'A';
    let x2 :&[u8;5] = b"hello";
    
    println!("{love}");
    println!("{chinese}");
    println!("{unicode1}");
    println!("{unicode2}");
    println!("{x1}");
    println!("{x2:?}");
}
