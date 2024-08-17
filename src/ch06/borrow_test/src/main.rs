fn main() {
    // example 1
    let x:&str ="hello";
    println!("{}", x); 
    let ptr = *&x;
    println!("{}", ptr); 
    println!("{}", x);
    
    // example 2
    let x:String ="hello".to_string();
    let len = calculate_length(&x);    
    println!("{} {}", x, len);
    
    // example 3
    let mut x:String ="hello".to_string();
    change(&mut x);
    println!("{}", x);
}

fn calculate_length(x: &String) -> usize {
    x.len()                              
}

fn change(x: &mut String) {
    x.push_str(", world");
}
