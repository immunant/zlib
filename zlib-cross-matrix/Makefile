# Builds two variants of the zlib API driver:
#   zlib-reference - linked against the system zlib (libz)
#   zlib-rust       - linked against the Rust translation's libzlib.so

CC ?= cc
CFLAGS ?= -O2 -g -Wall

ROOT := $(abspath ..)
RUST_DIR := $(ROOT)/translated_rust
RUST_LIB := $(RUST_DIR)/target/release/libzlib.so

.PHONY: all clean rust-lib

all: zlib-reference zlib-rust

zlib-reference: zlib-api-driver.c
	$(CC) $(CFLAGS) -o $@ $< -lz

zlib-rust: zlib-api-driver.c rust-lib
	$(CC) $(CFLAGS) -o $@ zlib-api-driver.c $(RUST_LIB)

rust-lib:
	cd $(RUST_DIR) && cargo build --release

clean:
	rm -f zlib-reference zlib-rust
