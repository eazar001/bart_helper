#!/usr/bin/env bash

cargo build --release --target x86_64-unknown-linux-musl --features vendored
zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap

echo "updating function code"
aws lambda update-function-code --function-name bart_info --no-publish --zip-file fileb://rust.zip --region us-west-2
