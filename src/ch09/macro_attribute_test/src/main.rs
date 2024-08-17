use macro_attribute_test::log;

#[log]
fn hello_world() {
    println!("Hello, world!");
}

fn main() {
    hello_world();
}
