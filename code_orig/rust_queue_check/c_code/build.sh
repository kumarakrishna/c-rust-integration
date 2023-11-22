#!/bin/sh

rm -f bin/*
# rustc --crate-type cdylib src/mylibrary.rs -o bin/libmylibrary.so
# clang src/main.c -Iinclude/ -Lbin -lmylibrary -o bin/myapp

cd ../rust_code/
cargo build --release
cd ../c_code/
gcc src/main.c -Iinclude/ -L../rust_code/target/release -lutillib -o bin/myapp