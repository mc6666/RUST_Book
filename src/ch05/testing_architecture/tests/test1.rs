// use testing_architecture;

// 成功案例
#[test]
fn test_add() {
    assert_eq!(testing_architecture::util::add(1, 2), 3);
}

// 失敗案例 1
#[test]
#[should_panic]
fn test_bad_add1() {
    let a: u8 = u8::MAX;
    let b: u8 = 1;
    let result: i32 = testing_architecture::util::add(a, b) as i32;
    assert_eq!(result , 256);
}

// 失敗案例 2
// #[test]
// fn test_bad_add2() {
    // assert_eq!(testing_architecture::util::add(1, 0-2), 255);
// }

// 失敗案例 3
#[test]
fn test_bad_add3() {
    assert_ne!(testing_architecture::util::add(1, 2) , 6); // 1+2 != 6
}
