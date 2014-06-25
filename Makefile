.PHONY	=all lib doctest test

all: lib

lib: src/*.rs
	rustc src/lib.rs

test: lib
	rustdoc -L. --test src/lib.rs
