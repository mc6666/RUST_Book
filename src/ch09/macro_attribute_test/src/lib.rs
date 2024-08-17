extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn log(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let function = parse_macro_input!(item as ItemFn);

    let name = &function.sig.ident;
    let inputs = &function.sig.inputs;
    let output = &function.sig.output;
    let block = &function.block;

    let expanded = quote! {
        fn #name(#inputs) #output {
            println!("Calling function: {}", stringify!(#name));
            #block
        }
    };

    TokenStream::from(expanded)
}
