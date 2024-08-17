#[tokio::main]
async fn main() {
    let mut x = 5;
    let task = tokio::spawn(async move {
        x = x + 1;
        println!("x={}", x);
        x
    });

    let x2 = task.await;
    println!("x2={}", x2.unwrap());
    println!("程式結束 !!");
}