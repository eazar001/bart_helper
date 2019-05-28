#!/usr/bin/env bash

if [[ -z $1 ]]; then
    echo "Building development binary"
    cargo build --target x86_64-unknown-linux-musl --features vendored && \
    zip -j rust.zip ./target/x86_64-unknown-linux-musl/debug/bootstrap && \

    echo "updating function code" && \
    aws lambda update-function-code --function-name bart_info --no-publish --zip-file fileb://rust.zip --region us-west-2
    exit 0
fi

if [[ $1 == "release" ]]; then
    echo "Building production binary"
    cargo build --release --target x86_64-unknown-linux-musl --features vendored && \
    zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap && \

    echo "updating function code" && \
    aws lambda update-function-code --function-name bart_info --no-publish --zip-file fileb://rust.zip --region us-west-2
else
    exit 1
fi
