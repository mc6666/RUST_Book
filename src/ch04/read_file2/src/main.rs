fn open_file(file_path:String) -> Result<String, std::io::Error> {
    let result = std::fs::read_to_string(file_path);
    return result;
}

fn main() {
    let text = open_file("data.txt".to_string()).unwrap();
    println!("{text:?}");
}
