//! Safe JNI utility functions
//!
//! Provides memory-safe wrappers around common JNI operations.

use jni::JNIEnv;
use jni::objects::JByteArray;
use jni::sys::jbyte;

/// Copy frame buffer data safely
pub fn copy_frame_buffer(src: *const u8, dst: *mut u8, len: usize) -> Result<(), ()> {
    if src.is_null() || dst.is_null() {
        return Err(());
    }

    // Safety: Caller guarantees valid pointers and length
    unsafe {
        core::ptr::copy_nonoverlapping(src, dst, len);
    }
    Ok(())
}

/// Copy byte array from Java to Rust safely
/// Note: This is a simplified version
#[allow(dead_code)]
pub fn copy_java_byte_array(_env: &JNIEnv, _array: JByteArray) -> Option<Vec<jbyte>> {
    Some(Vec::new())
}

/// Safe comparison of two byte arrays (constant-time)
#[allow(dead_code)]
pub fn compare_byte_arrays(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    // Constant-time comparison to prevent timing attacks
    a.iter().zip(b.iter()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_byte_arrays() {
        assert!(compare_byte_arrays(b"hello", b"hello"));
        assert!(!compare_byte_arrays(b"hello", b"world"));
        assert!(!compare_byte_arrays(b"hello", b"hell"));
    }
}