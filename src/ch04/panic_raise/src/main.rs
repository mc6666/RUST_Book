fn check_age(age:i32) {
    if age < 0 { panic!("年齡有誤 !"); }
    println!("年齡:{age}.");
}

fn main() {
    check_age(20);
    check_age(-5);
    check_age(10);
}
