fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    
    // read file
    let content = std::fs::read_to_string(path)
        .expect("could not read file");
    // println!("{}", content);
    
    // 逐行比對
    for (line_number, line) in content.lines().enumerate() {
        if line.contains(&pattern) {
            println!("{}: {}", line_number, line);
        }
    }
}
