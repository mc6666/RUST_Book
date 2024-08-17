@echo off

set RUSTFLAGS=-L..\c_cpp\build\Debug
cargo build
