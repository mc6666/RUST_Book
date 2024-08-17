fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

struct Foo<'a> {
    x: &'a i32,
}

fn main() {
    // test 1
    print_refs(&4,&9);
    
    // test 2, error
    let x;                    // -+ x goes into scope
    {                         //  |
        let y = &5;           // ---+ y goes into scope
        let f = Foo { x: y }; // ---+ f goes into scope
        x = &f.x;             //  | | error here
    }                         // ---+ f and y go out of scope
    println!("{}", x);        //  |
}             
