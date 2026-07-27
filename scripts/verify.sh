#!/bin/sh
set -eu

cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo build --locked --release
