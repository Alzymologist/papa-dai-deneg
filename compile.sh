#!/bin/bash

clear
# cargo watch -c -x run
#cargo watch -c -x "run -- 0x8094a91dc4d98a6112374c599d4ed6592a1862d7cda654ee74ecb649ca427a4c 99999 --please"

RUSTFLAGS='-C target-cpu=x86-64-v3'
#RUSTFLAGS=''
cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu --release


