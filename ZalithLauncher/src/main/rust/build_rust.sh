#!/bin/bash
# Build Rust library for Android
set -e

echo "==> Building Rust library for Android NDK..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "ERROR: Rust is not installed. Install from https://rustup.rs/"
    exit 1
fi

# Install cargo-ndk if not available
if ! cargo ndk --version &> /dev/null; then
    echo "Installing cargo-ndk..."
    cargo install cargo-ndk
fi

# Android NDK version (should match build.gradle.kts)
export NDK_VERSION=25.2.9519653
export ANDROID_NDK_HOME=${ANDROID_NDK_HOME:-$HOME/Android/Sdk/ndk/$NDK_VERSION}

if [ ! -d "$ANDROID_NDK_HOME" ]; then
    echo "ERROR: Android NDK not found at $ANDROID_NDK_HOME"
    echo "Set ANDROID_NDK_HOME environment variable or install NDK $NDK_VERSION"
    exit 1
fi

# Build for all Android ABIs
echo "Building for armeabi-v7a..."
cargo ndk --target armv7-linux-androideabi --platform 26 -- build --release

echo "Building for arm64-v8a..."
cargo ndk --target aarch64-linux-android --platform 26 -- build --release

echo "Building for x86..."
cargo ndk --target i686-linux-android --platform 26 -- build --release

echo "Building for x86_64..."
cargo ndk --target x86_64-linux-android --platform 26 -- build --release

# Copy libraries to jniLibs
echo "Copying libraries to jniLibs..."
mkdir -p ../../jniLibs/armeabi-v7a
mkdir -p ../../jniLibs/arm64-v8a
mkdir -p ../../jniLibs/x86
mkdir -p ../../jniLibs/x86_64

cp ../../../target/armv7-linux-androideabi/release/libzalith_rust.so ../../jniLibs/armeabi-v7a/
cp ../../../target/aarch64-linux-android/release/libzalith_rust.so ../../jniLibs/arm64-v8a/
cp ../../../target/i686-linux-android/release/libzalith_rust.so ../../jniLibs/x86/
cp ../../../target/x86_64-linux-android/release/libzalith_rust.so ../../jniLibs/x86_64/

# Generate C header
echo "Generating C header..."
if ! command -v cbindgen &> /dev/null; then
    echo "Installing cbindgen..."
    cargo install cbindgen
fi
cbindgen --config cbindgen.toml --crate zalith_rust --output include/zalith_rust.h

echo "==> Build complete! Rust libraries are in jniLibs/"
echo "==> C header is at include/zalith_rust.h"