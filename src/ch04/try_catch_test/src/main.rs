#![allow(unused)]

use try_catch::catch;
use std::*;
use serde_json::Value;

fn main() {
    catch! {
        try {
            let number: i32 = "10".parse()?;
            let data = fs::read_to_string("data.json")?;
            let json: Value = serde_json::from_str(&data)?;
        }
        catch error: io::Error {
            println!("找不到指定的檔案: {}", "data.json")
        }
        catch json_err: serde_json::Error {
            println!("Failed to serialize data: {}", json_err)
        }
        catch err {
            println!("Error of unknown type: {}", err)
        }
    };
}
