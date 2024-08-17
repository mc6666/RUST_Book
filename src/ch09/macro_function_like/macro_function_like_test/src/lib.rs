extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn print_and_replace(input: TokenStream) -> TokenStream {
    println!("Inputs: {}", input.to_string());
    "fn add(a:i32, b:i32) -> i32 { a+ b }".parse().unwrap()
}
