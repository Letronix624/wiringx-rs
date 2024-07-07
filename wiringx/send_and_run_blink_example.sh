#!/bin/bash

cargo +nightly build --examples --release --target=riscv64gc-unknown-linux-musl -Zbuild-std=std,core

scp -O ../target/riscv64gc-unknown-linux-musl/release/examples/blink root@192.168.42.1:
ssh root@192.168.42.1 /root/blink
