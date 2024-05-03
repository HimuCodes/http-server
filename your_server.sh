#!/bin/sh

exec cargo run \
    --quiet \
    --release \
    --target-dir=/tmp/codecrafters-http-server-target \
    --manifest-path $(dirname $0)/Cargo.toml -- "$@"
