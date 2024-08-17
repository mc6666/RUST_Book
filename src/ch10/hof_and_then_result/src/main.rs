#![allow(unused)]
use std::path::Path;
use chrono;

fn main() {
    // good test 
    let result = Path::new("src/main.rs").metadata()
                    .and_then(|md| md.modified());
    let _ = match result { 
        Ok(root_modified_time) => {
            let dt: chrono::DateTime<chrono::offset::Local> = root_modified_time.into();
            println!("{}", dt.format("%Y-%m-%d %H:%M:%S"))
        },
        Err(_) => println!("")
    };
    
    // bad test
    let result = Path::new("bad/main.rs").metadata()
                    .and_then(|md| md.modified());
    let _ = match result { 
        Ok(root_modified_time) => {
            let dt: chrono::DateTime<chrono::offset::Local> = root_modified_time.into();
            println!("{}", dt.format("%Y-%m-%d %H:%M:%S"));
        },
        Err(_) => println!("")
    };
}
