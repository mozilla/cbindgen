#!/usr/bin/env bash

set -xeu
cd "$(dirname "$0")/.."

export CC=gcc-8
export CXX=g++-8

# Not all nightlies have rustfmt, so we need to check for it.
RUSTFMT_TOOLCHAIN="nightly-$(curl https://rust-lang.github.io/rustup-components-history/$(rustup target list --installed | tail -1)/rustfmt)"
rustup update "$RUSTFMT_TOOLCHAIN"
rustup component add rustfmt --toolchain "$RUSTFMT_TOOLCHAIN"

cargo "+$RUSTFMT_TOOLCHAIN" fmt --all -- --check
cargo build --verbose
cargo build --verbose --no-default-features
CBINDGEN_TEST_VERIFY=1 cargo test --verbose
cargo build --verbose --release
cargo update -Zminimal-versions
cargo test
