use glob::glob;

fn main() {
    // method 1
    println!("method 1:");
    for entry in glob("*.*").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }
    
    // method 2
    println!("\nmethod 2:");
    for entry in glob("*").expect("Failed to read glob pattern") {
        println!("{:?}", entry.unwrap().display())
    }
}
