use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;  // Pancakes會指派給Macro內的name

fn main() {
    Pancakes::hello_macro();  // 呼叫Macro內的hello_macro
}
