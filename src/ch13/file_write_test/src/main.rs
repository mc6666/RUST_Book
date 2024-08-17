#![allow(unused)]

use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn simplest_write_file(filepath:&str, data: &[u8]) -> std::io::Result<()> {
    std::fs::write(filepath, &data)?;
    Ok(())
}

fn write_file(filepath:&str, data: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(filepath)?;
    file.write_all(&data)?;
    Ok(())
}

fn write_file_by_block(filepath:&str) -> std::io::Result<()> {
    let file = File::create(filepath)?;
    let mut stream = std::io::BufWriter::new(file);

    for i in 0..10 {
        stream.write(&[i+1])?;
    }
    Ok(())
}

fn read_file_by_block(filepath:&str) -> std::io::Result<()> {
    const BUFFER_LEN: usize = 1;
    let mut buffer = [0u8; BUFFER_LEN];
    let mut file = File::open(filepath)?;

    loop {
        let read_count = file.read(&mut buffer)?;
        if read_count != BUFFER_LEN {
            break;
        }
        let i: u8 = u8::from_le_bytes(buffer);
        println!("{i}");
    }
    Ok(())
}

fn main() {
    // simplest write file
    let _ = match simplest_write_file("data1.txt", b"Hello world!") {
        Err(err) => println!("{err}"),
        Ok(_) => ()
    };

    // write file, return string
    let _ = match write_file("data2.txt", b"Hello world!") {
        Err(err) => println!("{err}"),
        Ok(_) => ()
    };

    // write file per block
    let _ = match write_file_by_block("data3.txt") {
        Err(err) => println!("{err}"),
        Ok(_) => ()
    };
    
    // 驗證：read file per block
    let _ = match read_file_by_block("data3.txt") {
        Err(err) => println!("{err}"),
        Ok(_) => ()
    };
    
}
