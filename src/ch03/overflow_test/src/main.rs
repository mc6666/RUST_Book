fn add(m: i8, n: i8) {
    println!("{}", m + n); // 溢位
}

fn main() {
    let m : i8 = 120;
    let n : i8 = 120;
    add(m, n);
}
