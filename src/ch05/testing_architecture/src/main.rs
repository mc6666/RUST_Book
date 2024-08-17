// mod lib { pub mod common_function; }
mod util;

fn main() {
    // let result: u8 = lib::common_function::add(1, 2);
    let result: u8 = util::add(1, 2);
    println!("{result}");
}
