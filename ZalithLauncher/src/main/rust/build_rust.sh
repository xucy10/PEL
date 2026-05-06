#!/bin/bash
# Build Rust library for Android using NDK
# Usage: ./build_rust.sh [ndk_path]

set -e

# Configuration
TARGET_DIR="jniLibs"
ARCHS=("arm64-v8a" "armeabi-v7a" "x86" "x86_64")

# Find NDK
if [ -n "$1" ]; then
    NDK_PATH="$1"
elif [ -n "$ANDROID_NDK_HOME" ]; then
    NDK_PATH="$ANDROID_NDK_HOME"
elif [ -n "$NDK_HOME" ]; then
    NDK_PATH="$NDK_HOME"
else
    echo "Error: NDK path not found. Set ANDROID_NDK_HOME or pass as argument."
    exit 1
fi

echo "Using NDK: $NDK_PATH"

# Create target directory
mkdir -p "$TARGET_DIR"

# Build for each architecture
for ARCH in "${ARCHS[@]}"; do
    echo "Building for $ARCH..."

    # Set up environment for this architecture
    case "$ARCH" in
        arm64-v8a)
            TARGET="aarch64-linux-android"
            ;;
        armeabi-v7a)
            TARGET="arm-linux-androideabi"
            ;;
        x86)
            TARGET="i686-linux-android"
            ;;
        x86_64)
            TARGET="x86_64-linux-android"
            ;;
    esac

    # Build
    CARGO_TARGET="$TARGET" cargo ndk -t "$ARCH" build --release

    # Copy output
    mkdir -p "$TARGET_DIR/$ARCH"
    cp "target/$TARGET/release/libzalith_jni.so" "$TARGET_DIR/$ARCH/" 2>/dev/null || \
    cp "target/aarch64-linux-android/release/libzalith_jni.so" "$TARGET_DIR/$ARCH/" 2>/dev/null || \
    echo "Warning: Could not find output for $ARCH"
done

echo "Build complete!"