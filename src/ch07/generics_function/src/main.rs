fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}

fn max<T: std::cmp::PartialOrd>(a:T, b:T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn get_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    // add test
    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));
    println!("add f64: {}", add(1.23, 1.23));

    // max test
    println!("max i8: {}", max(2i8, 3i8));
    println!("max i32: {}", max(20, 30));
    println!("max f64: {}", max(1.23, 1.23));
    
    // get_largest test for number
    let number_list = vec![34, 50, 25, 100, 65];
    let result = get_largest(&number_list);
    println!("The largest number is {}", result);

    // get_largest test for char
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = get_largest(&char_list);
    println!("The largest char is {}", result);
    
}
