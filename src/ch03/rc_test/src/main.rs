use std::rc::Rc;

fn main() {
    let a = Rc::new(5);
    // Rc::strong_count 取得參考計數
    println!("建立 a 後的計數 = {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("建立 b 後的計數 = {}", Rc::strong_count(&a));
    {
        let c = Rc::clone(&a);
        println!("建立 c 後的計數 = {}", Rc::strong_count(&a));
    }
    
    // 另一種表示法
    let d = a.clone();
    println!("建立 d 後的計數 = {}", Rc::strong_count(&a));
    
    println!("{}", d);

}
