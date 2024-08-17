use std::path::Path;
use cmake::Config;

fn main()
{
    let dst = Config::new(Path::new("..").join("c_cpp")).build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=dylib=c_cpp");
}
