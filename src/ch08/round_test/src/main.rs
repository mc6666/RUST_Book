use round::round;

fn main() {
    println!("{}", round(3.5, 0));
    println!("{}", round(3.51, 0));
    println!("{}", round(3.52, 0));
    println!("{}", round(3.551, 1));
    println!("{}", round(3.552, 1));
}
