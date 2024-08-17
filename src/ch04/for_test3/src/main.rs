fn main() {
    let names = ["John", "Mary", "Tom"];
    for (index,val) in names.iter().enumerate() {
        println!("index = {index} and val = {val}");
    }
    
    // 字串處理
    let lines = "hello\nworld".lines(); // 分行
    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }
}
