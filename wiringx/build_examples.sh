#!/bin/bash

cargo +nightly build --examples --release --target=riscv64gc-unknown-linux-musl -Zbuild-std=std,core

