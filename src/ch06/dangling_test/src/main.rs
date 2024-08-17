fn main() {
    // let x = dangle();
    let x = not_dangle();
	println!("{x}");
}

// fn dangle() -> &String {
    // let s = String::from("hello");
    // &s
// }

// static S: &str = "hello";
// fn not_dangle() -> &'static str {
    // &S
// }

fn  not_dangle<'a>() -> &'a str {
    let s:&str = "hello";
    &s
}
