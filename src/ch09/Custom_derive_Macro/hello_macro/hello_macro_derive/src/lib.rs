use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 建立語法樹
    let ast = syn::parse(input).unwrap();

    // 呼叫 impl_hello_macro，實現 HelloMacro trait
    impl_hello_macro(&ast)
}

// 實現HelloMacro trait
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident; // 取得宣告的物件：struct
    let gen = quote! {     // quote 將變數置換為物件屬性
        impl HelloMacro for #name {  // #name：物件名稱
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

