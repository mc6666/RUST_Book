// Box smart pointer
fn main() {
    let data = 20;       
    let x = Box::new(data); 

    println!("x={}",x);
    println!("*x={}",*x);
}
