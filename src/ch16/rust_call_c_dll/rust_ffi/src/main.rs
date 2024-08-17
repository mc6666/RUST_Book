use std::error::Error;

use rust_ffi::introduce_rust;

fn main() -> Result<(), Box<dyn Error>> {
    let introduction = introduce_rust("Srikanth", 31)?;
    println!("{introduction}");
    Ok(())
}
