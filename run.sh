#!/usr/bin/zsh

cargo new crates/thresh-protocol --lib
cargo new crates/thresh-trie --lib
cargo new crates/thresh-storage --lib
cargo new crates/thresh-sync --lib
cargo new crates/thresh-search --lib
cargo new crates/thresh-daemon --bin
cargo new crates/thresh-cli --bin
cargo new crates/thresh-api --bin
cargo new crates/thresh-wasm --lib