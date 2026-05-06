//! Utility functions


/// Safe memory copy for frame buffer
/// Returns Ok(()) on success, Err(errno) on failure
#[inline(always)]
pub fn copy_frame_buffer(src: *const u8, dst: *mut u8, len: usize) -> Result<(), i32> {
    unsafe {
        // Simple byte-by-byte copy for safety
        // In production, this could use SIMD if available
        for i in 0..len {
            *dst.add(i) = *src.add(i);
        }
    }
    Ok(())
}

// Cross-platform errno access
// Note: On Android bionic, errno is thread-local and accessed differently
// For simplicity, we use the return value approach where possible
#[inline(always)]
pub fn get_errno() -> i32 {
    // When errno is needed, we get it from the return value of the syscall
    // This is typically -1 and the actual errno is in thread-local storage
    // Since we can't easily access it cross-platform, return 0 as fallback
    // The caller should check the actual syscall return value instead
    0
}