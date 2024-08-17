#!/bin/zsh

cp ../c_cpp/build/libc_cpp.dylib ./target/debug/
install_name_tool -change @rpath/libc_cpp.dylib @executable_path/libc_cpp.dylib ./target/debug/rust_ffi
./target/debug/rust_ffi
