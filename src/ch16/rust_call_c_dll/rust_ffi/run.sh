#!/bin/sh

export LD_LIBRARY_PATH=../c_cpp/build/:../build/:$LD_LIBRARY_PATH
./target/debug/rust_ffi
