#![allow(unused)]

use std::fs;
use std::path::Path;

fn main() {
    const TEST_FOLDER_STR: &str = "./test/";

    // check test folder exist
    if Path::new(&TEST_FOLDER_STR).exists() {
        fs::remove_dir(TEST_FOLDER_STR);
    }
    fs::create_dir(TEST_FOLDER_STR);
    
    
    let test_folder: String = TEST_FOLDER_STR.to_string();
    
    // copy file
    fs::copy("data1.txt", test_folder.clone() + "tmp.txt");
    
    // remove file
    fs::remove_file(test_folder.clone() + "tmp.txt");
    
    fs::copy("data1.txt", test_folder.clone() + "tmp.txt");
    // rename file
    fs::rename(test_folder.clone() + "tmp.txt", test_folder.clone() + "data.txt");
    
    // move file
    fs::rename(test_folder.clone() + "data.txt", test_folder.clone() + "data1.txt");
    
    // print file attributes
    let attr = fs::metadata("data1.txt").unwrap();
    println!("{:?}", attr);
    println!("{}", attr.len());
    println!("{}", attr.file_type().is_dir());
    
    // print file attributes
    let attr = fs::metadata("test").unwrap();
    println!("\n{:?}", attr);
    println!("{}", attr.len());
    println!("{}", attr.file_type().is_dir());
    
}
