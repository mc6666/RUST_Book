#![allow(unused)]

use std::{fs, io};
use std::fs::DirEntry;
use std::path::Path;

fn get_files(path: &Path) -> Vec<String> {
    let result = fs::read_dir(path).unwrap();

    let mut files: Vec<String> = Vec::new();

    // cb：匿名函數，取得所有檔案名稱
    let mut cb = |entry: &DirEntry| {
        // 儲存所有檔案名稱
        let path = entry.path();
        if path.is_file() {
            files.push(path.into_os_string().into_string().unwrap())
        }
    };
    visit_dirs(path, &mut cb);
    files
}

// 遞迴掃描目錄
fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn list_dir(dir: &str) -> std::io::Result<Vec<String>> {
    let mut contents = vec![];
    for entry in fs::read_dir(dir)? {
        let dir = entry?;
        contents.push(dir.path().into_os_string().into_string().unwrap());
    }
    Ok(contents)
}

fn main() {
    const TEST_FOLDER_STR: &str = "./test/test2/test3";

    // check test folder exist
    if Path::new("./test").exists() {
        // 刪除目錄及其下檔案與子目錄
        fs::remove_dir_all("./test");
    }
    
    // 建立目錄及其下子目錄
    fs::create_dir_all(TEST_FOLDER_STR);
    // 只能建立一層目錄
    fs::create_dir("./test/test1");
    
    // 掃描一層目錄
    let mut entries = fs::read_dir(".\\test").unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>().unwrap();
    entries.sort();
    println!("{:?}\n", entries);
    
    // 掃描一層目錄
    let mut entries = list_dir(".\\test").unwrap();
    println!("{:?}\n", entries);
    
    // 測試：複製檔案
    fs::copy("data1.txt", "./test/test1/data1.txt");
    fs::copy("data2.txt", "./test/test2/data2.txt");
    fs::copy("data3.txt", "./test/test2/test3/data3.txt");

    // 掃描目錄及其下子目錄
    let files = get_files(Path::new(".\\test"));
    println!("{:?}", files);
    

    
    
}
