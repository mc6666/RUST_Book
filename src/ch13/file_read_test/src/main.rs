#![allow(unused)]

use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io::Error;

fn simplest_read_file(filepath:&str) -> Result<Vec<u8>, Error> {
    let data = std::fs::read(filepath)?;
    Ok(data)
}

fn read_file(filepath:&str) -> Result<String, Error> {
    let mut file = File::open(filepath)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

fn read_file_by_line(filepath:&str) -> Result<Vec<String>, Error> {
    let file = File::open(filepath)?;
    let reader = std::io::BufReader::new(file);

    let mut contents = vec![];
    for line in reader.lines() {
        contents.push(line?);
    }
    Ok(contents)
}

fn read_file_by_block(filepath:&str) -> Result<Vec<u8>, Error> {
    const BUFFER_LEN: usize = 512;
    let mut buffer = [0u8; BUFFER_LEN];
    let mut file = File::open(filepath)?;

    let mut contents = vec![];
    loop {
        let read_count = file.read(&mut buffer)?;
        // extend_from_slice：將陣列附加至另一陣列後面
        // buffer[..read_count]：一次讀取的內容
        contents.extend_from_slice(&buffer[..read_count]);

        // 讀取的長度不足，表示已讀至檔尾
        if read_count != BUFFER_LEN {
            break;
        }
    }
    Ok(contents)
}

fn main() {
    // simplest read file
    let _ = match simplest_read_file("data.txt") {
        Err(err) => println!("{err}"),
        Ok(data) => {
            let data2 = String::from_utf8(data).unwrap(); 
            println!("{}\n", data2)
        }
    };

    // read file, return string
    let _ = match read_file("data.txt") {
        Err(err) => println!("{err}"),
        Ok(data) => println!("{}\n", data)
    };

    // read file, return vector
    let _ = match read_file_by_line("data.txt") {
        Err(err) => println!("{err}"),
        Ok(data) => println!("{}\n", data.join("\n"))
    };

    // read file per block
    let _ = match read_file_by_block("data.txt") {
        Err(err) => println!("{err}"),
        Ok(data) => {
            let data2 = String::from_utf8(data).unwrap(); 
            println!("{}\n", data2)
        }
    };
}
