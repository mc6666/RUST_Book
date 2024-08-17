pub fn add(a: u8, b: u8) -> u8 {
    a + b
}

fn main() {
    let result: u8 = add(1, 2);
    println!("{result}");
    
    // 溢位(overflow)
    let result: u8 = add(u8::MAX, 1);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    // 加這一行才能看的見外部函數
    use super::*;

    // 成功案例
    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    // 失敗案例 1
    #[test]
    #[should_panic]
    fn test_bad_add1() {
        let a: u8 = u8::MAX;
        let b: u8 = 1;
        let result: i32 = add(a, b) as i32;
        assert_eq!(result , 256);
    }

    // 失敗案例 2
    // #[test]
    // fn test_bad_add2() {
        // assert_eq!(add(1, 0-2), 255);
    // }

    // 失敗案例 3
    #[test]
    fn test_bad_add3() {
        assert_ne!(add(1, 2) , 6); // 1+2 != 6
    }
}


