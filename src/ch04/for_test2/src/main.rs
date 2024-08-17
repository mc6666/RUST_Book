fn main() {
    let mut sum = 0;
    for i in (0..101).step_by(2) {
       sum+=i;
    }
    println!("sum:{sum}");
    
    // 另一種寫法
    let mut sum = 0;
    for i in 0..101 {
        if i % 2 == 1 { continue; }
       sum+=i;
    }
    println!("sum:{sum}");
}