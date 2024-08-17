#![allow(unused)]

use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::error::Error;

fn create_file(file_name:&str) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}

fn open_file(file_name:&str) -> std::io::Result<()> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}

fn open_file_vec(file_name:&str) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(file_name)?;
    let mut contents = vec![];
    file.read_to_end(&mut contents)?;
    println!("{:?}", contents);
    let s = std::str::from_utf8(&contents)?; 
    println!("{}", s);
    Ok(())
}

fn flush_file(file_name:&str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true)
                    .create(true).open(file_name)?;
    for _ in 0..10000 {
        file.write_all(b"Hello, world!")?;
    }
    println!("\n{:?}", file.metadata().unwrap().len()); 
    file.sync_all()?;
    println!("\n{:?}\n", file.metadata().unwrap().len());
    Ok(())
}

fn main() {
    // create file
    let _ = match create_file("foo.txt") {
        Err(err) => println!("{err}"),
        Ok(_) => ()
    };

    // flush file
    let _ = match flush_file("data.txt") {
        Err(err) => println!("{err}"),
        Ok(_) => ()
    };

    // open file
    let _ = match open_file("foo.txt") {
        Err(err) => println!("{err}"),
        Ok(_) => ()
    };

    // open file 2
    let _ = match open_file_vec("foo.txt") {
        Err(err) => println!("{err}"),
        Ok(_) => ()
    };
}
