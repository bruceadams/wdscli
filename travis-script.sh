#!/bin/bash

set -euxo pipefail

cargo_target=''
if [[ "$TRAVIS_OS_NAME" == "linux" ]]; then
    cargo_target='--target x86_64-unknown-linux-musl'
fi

~/.cargo/bin/cargo build --verbose --release $cargo_target
