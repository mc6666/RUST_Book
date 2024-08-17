fn main() {
    // 變數 small 初始化為一個非常小的浮點數
    let mut small = std::f32::EPSILON;
    // 不斷迴圈，讓 small 越來越趨近於 0，直到最後等於0的狀態
    while small > 0.0 {
        small = small / 2.0;
        println!("{} {:?}", small, small.classify());
    }
}
