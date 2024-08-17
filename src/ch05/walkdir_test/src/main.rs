use walkdir::WalkDir;

fn main() {
    // method 1
    println!("method 1:");
    for e in WalkDir::new(".\\src").into_iter().filter_map(|e| e.ok())
        .filter(|e| match e.path().extension() {
            None => false,
            Some(ex) => ex == "rs"})
    {
        if e.metadata().unwrap().is_file() {
            println!("{}", e.path().display());
        }
    }

    // method 2
    // println!("\nmethod 2:");
    // for file in WalkDir::new(".").into_iter().filter_map(|file| file.ok()) {
        // println!("{}", file.path().display());
    // }
}
