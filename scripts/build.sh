#!/bin/bash

run_script() {
    echo -e "\033[0;36mRunning \`$*\`\033[0m"
    "$@"
}

echo "Building static libraries..."
run_script cargo build --release --target aarch64-apple-ios
run_script cargo build --release --target aarch64-apple-ios-sim

# TODO: create fat files using lipo

echo "Creating package"
run_script swift-bridge-cli create-package \
    --bridges-dir ./generated \
    --out-dir . \
    --simulator ./target/aarch64-apple-ios-sim/release/libnorg_lib.a \
    --ios ./target/aarch64-apple-ios/release/libnorg_lib.a \
    --name SwiftyNorg
