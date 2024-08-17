use macro_function_like_test::print_and_replace;

print_and_replace!(100); // 引進巨集

fn main() {
    println!("{}", add(1,2)); // 呼叫巨集定義的函數add
}
