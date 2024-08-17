use std::thread;
use std::path::Path;
use std::time::Duration;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
// encoding_rs = "0.8.33"
use encoding_rs::BIG5;

// 取得資料型別
fn print_type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
fn sleep(n:i32) {
    thread::sleep(Duration::from_secs(n));
}

fn now(is_local:bool) {
    let curr_time = std::time::SystemTime::now();
    if is_local {
        let dt: chrono::DateTime<chrono::offset::Local> = curr_time.clone().into();
        dt.format("%Y-%m-%d %H:%M:%S")
    } else {
        // UTC
        let dt: chrono::DateTime<chrono::offset::Utc> = curr_time.clone().into();
        dt.format("%Y-%m-%d %H:%M:%S %P %z")
    }
}

fn print_BIG5(data:Bytes) {
    // print BIG5-encoding data
    let (encoding_text, encoding_used, had_errors) = BIG5.decode(&data);
    println!("Body:\n{}", encoding_text);
}

fn convert_BIG5_to_UTF8(src:&str, dest:&str) {
    // Big 轉存 UTF-8 
    let data = std::fs::read(src).unwrap();
    let decoding_data = encoding::all::BIG5_2003.decode(&data, DecoderTrap::Strict).unwrap();
    std::fs::write(dest, decoding_data);
    // println!("{}", decoding_data);
}

fn get_file_modified_time(path:&str) {
    let result = Path::new(str).metadata()
                    .and_then(|md| md.modified());
    let _ = match result { 
        Ok(root_modified_time) => {
            let dt: chrono::DateTime<chrono::offset::Local> = root_modified_time.into();
            dt.format("%Y-%m-%d %H:%M:%S"))
        },
        Err(msg) => format!("ERROR:{msg}")
    };
}

