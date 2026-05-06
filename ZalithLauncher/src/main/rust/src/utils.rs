//! Utility functions

use libc::size_t;

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

/// Get last error number
#[inline(always)]
pub fn get_errno() -> i32 {
    unsafe { *libc::__errno_location() }
}