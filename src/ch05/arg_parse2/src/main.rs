struct Args {
    pattern: String,
    path: std::path::PathBuf,
    n:u8
}

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    let n:u8 = std::env::args().nth(3).expect("no n given")
                .trim().parse().expect("not numeric");
    
    let args = Args {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
        n: n,
    };
    
    println!("{}, {:?}, {}", args.pattern, args.path, args.n);
}
