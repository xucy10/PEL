//! SIMD utilities for frame buffer operations
//!
//! Platform-specific SIMD acceleration for memory copy operations.
//! Uses SSE2 on x86_64 and NEON on aarch64 when available.

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_storeu_si128, _mm_cvtsi128_si32};

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::{float32x4_t, vld1q_f32, vst1q_f32};

/// Get optimal SIMD copy size (16 bytes for SSE2, 16 bytes for NEON)
#[inline(always)]
pub fn get_simd_optimal_size() -> usize {
    16
}

/// SIMD-accelerated frame buffer copy
/// Returns number of bytes copied, or -1 on error
#[cfg(target_arch = "x86_64")]
pub unsafe fn simd_copy_frame_buffer(src: *const u8, dst: *mut u8, len: usize) -> i32 {
    let mut i = 0;

    // Align to 16 bytes
    while i < len && ((dst.add(i) as usize) & 15) != 0 {
        *dst.add(i) = *src.add(i);
        i += 1;
    }

    // SIMD loop - process 16 bytes at a time
    let simd_end = (len - i) / 16 * 16 + i;
    while i < simd_end {
        let src_ptr = src.add(i) as *const __m128i;
        let dst_ptr = dst.add(i) as *mut __m128i;
        _mm_storeu_si128(dst_ptr, _mm_loadu_si128(src_ptr));
        i += 16;
    }

    // Handle remaining bytes
    while i < len {
        *dst.add(i) = *src.add(i);
        i += 1;
    }

    len as i32
}

#[cfg(target_arch = "aarch64")]
pub unsafe fn simd_copy_frame_buffer(src: *const u8, dst: *mut u8, len: usize) -> i32 {
    let mut i = 0;

    // Align to 16 bytes
    while i < len && ((dst.add(i) as usize) & 15) != 0 {
        *dst.add(i) = *src.add(i);
        i += 1;
    }

    // NEON loop - process 16 bytes at a time
    let simd_end = (len - i) / 16 * 16 + i;
    while i < simd_end {
        let src_ptr = src.add(i) as *const float32x4_t;
        let dst_ptr = dst.add(i) as *mut float32x4_t;
        vst1q_f32(dst_ptr, vld1q_f32(src_ptr));
        i += 16;
    }

    // Handle remaining bytes
    while i < len {
        *dst.add(i) = *src.add(i);
        i += 1;
    }

    len as i32
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
pub unsafe fn simd_copy_frame_buffer(src: *const u8, dst: *mut u8, len: usize) -> i32 {
    // Fallback for other architectures
    for i in 0..len {
        *dst.add(i) = *src.add(i);
    }
    len as i32
}