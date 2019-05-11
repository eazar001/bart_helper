#!/usr/bin/env bash

cargo build --release --target x86_64-unknown-linux-musl --features vendored
zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap
