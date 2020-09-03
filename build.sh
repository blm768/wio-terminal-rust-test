#!/bin/sh

set -e

cargo build --release
cargo objcopy --release -- -O binary wio-terminal-rust-test.bin
bossac --erase --write --boot --reset --offset=0x4000 wio-terminal-rust-test.bin
