// 1個參數
macro_rules! say_something {
    ($x:expr) => {
        println!("{}", $x);
    };
}

// 2個參數
macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

fn main() {
    say_something!("Hello, world!");
    
    // add test
    let result = add!(1, 2);
    println!("{}", result);
}
