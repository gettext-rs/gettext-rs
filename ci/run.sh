#!/bin/sh

set -ex

cargo test --package gettext-sys warnings --target $TARGET --no-run -vv -- -D warnings
cargo test --package gettext-rs --target $TARGET --no-run -vv -- -D warnings
# We don't deny warnings here because we don't care about warnings in auto-generated code.
cargo build --package systest --target $TARGET -vv

if [ -z "$NO_RUN" ]; then
    if [ -n "$GETTEXT_LIB_DIR" ]; then
      export LD_LIBRARY_PATH=${GETTEXT_LIB_DIR}:$LD_LIBRARY_PATH
    elif [ -n "$GETTEXT_DIR" ]; then
      export LD_LIBRARY_PATH=${GETTEXT_DIR}/lib:$LD_LIBRARY_PATH
    fi
    cargo test --exclude systest --workspace --target $TARGET --verbose -- --nocapture
    cargo doc --no-deps --target $TARGET
    # We don't deny warnings here because we don't care about warnings in auto-generated code.
    cargo run --manifest-path systest/Cargo.toml --target $TARGET -vv
fi

if [ -n "$FEATURES" ]
then
    # We don't deny warnings here because we don't care about warnings in auto-generated code.
    cargo run --manifest-path systest/Cargo.toml --target $TARGET --features "$FEATURES"
fi
