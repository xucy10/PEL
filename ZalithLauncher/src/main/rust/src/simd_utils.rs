//! SIMD-optimized utility functions using ARM NEON / x86 SSE
//!
//! Provides high-performance operations for:
//! - Frame buffer copying with NEON/SSE
//! - Memory operations
//! - Texture data processing

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use libc::{c_void, size_t};

/// Copy frame buffer using SIMD (NEON on ARM, SSE on x86_64)
/// Returns number of bytes copied, or -1 on error
#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub unsafe fn simd_copy_frame_buffer(src: *const u8, dst: *mut u8, len: usize) -> isize {
    if src.is_null() || dst.is_null() || len == 0 {
        return -1;
    }

    let mut remaining = len;
    let mut src_ptr = src;
    let mut dst_ptr = dst;

    // Process 64 bytes at a time (NEON q-register)
    while remaining >= 64 {
        let data0: uint8x16_t = vld1q_u8(src_ptr);
        let data1: uint8x16_t = vld1q_u8(src_ptr.add(16));
        let data2: uint8x16_t = vld1q_u8(src_ptr.add(32));
        let data3: uint8x16_t = vld1q_u8(src_ptr.add(48));

        vst1q_u8(dst_ptr, data0);
        vst1q_u8(dst_ptr.add(16), data1);
        vst1q_u8(dst_ptr.add(32), data2);
        vst1q_u8(dst_ptr.add(48), data3);

        src_ptr = src_ptr.add(64);
        dst_ptr = dst_ptr.add(64);
        remaining -= 64;
    }

    while remaining >= 16 {
        let data: uint8x16_t = vld1q_u8(src_ptr);
        vst1q_u8(dst_ptr, data);
        src_ptr = src_ptr.add(16);
        dst_ptr = dst_ptr.add(16);
        remaining -= 16;
    }

    while remaining > 0 {
        *dst_ptr = *src_ptr;
        src_ptr = src_ptr.add(1);
        dst_ptr = dst_ptr.add(1);
        remaining -= 1;
    }

    len as isize
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn simd_copy_frame_buffer(src: *const u8, dst: *mut u8, len: usize) -> isize {
    if src.is_null() || dst.is_null() || len == 0 {
        return -1;
    }

    let mut remaining = len;
    let mut src_ptr = src;
    let mut dst_ptr = dst;

    while remaining >= 32 {
        let data0 = _mm_loadu_si128(src_ptr as *const __m128i);
        let data1 = _mm_loadu_si128(src_ptr.add(16) as *const __m128i);

        _mm_storeu_si128(dst_ptr as *mut __m128i, data0);
        _mm_storeu_si128(dst_ptr.add(16) as *mut __m128i, data1);

        src_ptr = src_ptr.add(32);
        dst_ptr = dst_ptr.add(32);
        remaining -= 32;
    }

    while remaining >= 16 {
        let data = _mm_loadu_si128(src_ptr as *const __m128i);
        _mm_storeu_si128(dst_ptr as *mut __m128i, data);
        src_ptr = src_ptr.add(16);
        dst_ptr = dst_ptr.add(16);
        remaining -= 16;
    }

    while remaining > 0 {
        *dst_ptr = *src_ptr;
        src_ptr = src_ptr.add(1);
        dst_ptr = dst_ptr.add(1);
        remaining -= 1;
    }

    len as isize
}

#[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
#[inline(always)]
pub unsafe fn simd_copy_frame_buffer(src: *const u8, dst: *mut u8, len: usize) -> isize {
    if src.is_null() || dst.is_null() || len == 0 {
        return -1;
    }
    core::ptr::copy_nonoverlapping(src, dst, len);
    len as isize
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub unsafe fn simd_memset(dst: *mut u8, value: u8, len: usize) -> isize {
    if dst.is_null() || len == 0 {
        return -1;
    }

    let pattern = vdupq_n_u8(value);
    let mut remaining = len;
    let mut dst_ptr = dst;

    while remaining >= 64 {
        vst1q_u8(dst_ptr, pattern);
        vst1q_u8(dst_ptr.add(16), pattern);
        vst1q_u8(dst_ptr.add(32), pattern);
        vst1q_u8(dst_ptr.add(48), pattern);
        dst_ptr = dst_ptr.add(64);
        remaining -= 64;
    }

    while remaining >= 16 {
        vst1q_u8(dst_ptr, pattern);
        dst_ptr = dst_ptr.add(16);
        remaining -= 16;
    }

    while remaining > 0 {
        *dst_ptr = value;
        dst_ptr = dst_ptr.add(1);
        remaining -= 1;
    }

    len as isize
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub unsafe fn simd_memset(dst: *mut u8, value: u8, len: usize) -> isize {
    if dst.is_null() || len == 0 {
        return -1;
    }
    core::ptr::write_bytes(dst, value, len);
    len as isize
}

/// Prefetch memory hint - on ARM64 this is a hint instruction that may be ignored
#[inline(always)]
pub fn prefetch_read(ptr: *const c_void, _rw: i32, _locality: i32) {
    let _ = ptr;
    // Prefetch is just a hint, the CPU will handle it appropriately
    // On ARM64 we could use DC/IC instructions but they require privileged mode
}

/// Memory barrier using atomic operations
#[inline(always)]
pub fn memory_barrier() {
    // Use SeqCst fence which is a full memory barrier on all architectures
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
}

/// Get optimal buffer size for SIMD operations
pub fn get_simd_optimal_size() -> size_t {
    #[cfg(target_arch = "aarch64")]
    return 64;

    #[cfg(target_arch = "x86_64")]
    return 32;

    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    return 16;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simd_copy_frame_buffer() {
        let src: [u8; 256] = (0..256).map(|i| i as u8).collect::<Vec<_>>().try_into().unwrap();
        let mut dst = [0u8; 256];

        unsafe {
            let result = simd_copy_frame_buffer(src.as_ptr(), dst.as_mut_ptr(), 256);
            assert_eq!(result, 256);
        }

        assert_eq!(&src[..], &dst[..]);
    }

    #[test]
    fn test_simd_memset() {
        let mut buffer = [0u8; 128];

        unsafe {
            simd_memset(buffer.as_mut_ptr(), 0xAB, 128);
        }

        assert!(buffer.iter().all(|&b| b == 0xAB));
    }

    #[test]
    fn test_get_simd_optimal_size() {
        let size = get_simd_optimal_size();
        assert!(size >= 16);
    }
}