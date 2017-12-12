#!/bin/sh

set -ex

cargo test --target $TARGET --no-run -vv
if [ -z "$NO_RUN" ]; then
    cargo test --target $TARGET --verbose -- --nocapture
    cargo run --manifest-path systest/Cargo.toml --target $TARGET -vv
    cargo doc --no-deps --target $TARGET
    cargo doc --no-deps -p gettext-sys --target $TARGET
fi

if [ -n "$FEATURES" ]
then
    cargo run --manifest-path systest/Cargo.toml --target $TARGET --features "$FEATURES"
fi
