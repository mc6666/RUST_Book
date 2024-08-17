#![allow(unused)]

use config::Config;
use std::collections::HashMap;

fn main() {
    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("./Settings"))
        .build()
        .unwrap();
                   
    // 顯示所有組態變數
    println!("HashMap: {:#?}", settings.clone()
        .try_deserialize::<HashMap<String, String>>()
        .unwrap());
        
    // 讀取並顯示單一變數
    println!("max_count: {}", settings.get::<i32>("max_count").unwrap());
    println!("name: {}", settings.get::<String>("name").unwrap());
}
