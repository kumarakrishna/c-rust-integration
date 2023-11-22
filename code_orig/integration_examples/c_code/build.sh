#!/bin/sh

rm -f bin/*
# rustc --crate-type cdylib src/mylibrary.rs -o bin/libmylibrary.so
# clang src/main.c -Iinclude/ -Lbin -lmylibrary -o bin/myapp


gcc src/main.c -Iinclude/ -L../rustcode/target/release -lutillib -o bin/myapp