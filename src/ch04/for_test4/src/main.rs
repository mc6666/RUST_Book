fn main() {
    // 字串使用迴圈分割成字元
    let s = "中文測試!";
    for c in s.chars() {
        println!("{c}");
    }

    println!("");
    // 字串使用迴圈分割成bytes
    for b in s.bytes() {
        println!("{b}");
    }
}
