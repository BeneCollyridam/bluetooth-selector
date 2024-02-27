#!/usr/bin/env bash
cargo build --release
mv target/x86_64-unknown-linux-musl/release/bluetooth-selector ~/.local/bin/
