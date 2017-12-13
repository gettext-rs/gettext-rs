#!/bin/sh

set -ex

cargo test --target $TARGET --no-run -vv
if [ -z "$NO_RUN" ]; then
    if [ -n "$GETTEXT_LIB_DIR" ]; then
      export LD_LIBRARY_PATH=${GETTEXT_LIB_DIR}:$LD_LIBRARY_PATH
    elif [ -n "$GETTEXT_DIR" ]; then
      export LD_LIBRARY_PATH=${GETTEXT_DIR}/lib:$LD_LIBRARY_PATH
    fi
    cargo test --target $TARGET --verbose -- --nocapture
    cargo run --manifest-path systest/Cargo.toml --target $TARGET -vv
    cargo doc --no-deps --target $TARGET
    cargo doc --no-deps -p gettext-sys --target $TARGET
fi

if [ -n "$FEATURES" ]
then
    cargo run --manifest-path systest/Cargo.toml --target $TARGET --features "$FEATURES"
fi
