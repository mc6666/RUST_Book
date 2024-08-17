fn main() {
    // Rust會記憶用過的資料型別
    let env_closure = |x| x;

    let s = env_closure(String::from("hello"));
    let n = env_closure(5);  // error

    // 將上述兩行程式碼對調
    let n = env_closure(5);
    let s = env_closure(String::from("hello"));  // error
}
