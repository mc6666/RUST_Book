#!/bin/sh

export RUSTFLAGS="-L../c_cpp/build -L../build"
cargo build
