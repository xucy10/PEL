//! ZalithLauncher Rust JNI Library
//!
//! Memory-safe implementations of critical native functions.
//! All functions can be called from Java/Kotlin via JNI.

#![allow(dead_code)]

pub mod bigcore;
pub mod driver_helper;
pub mod environ;
pub mod exit_hook;
pub mod file_io;
pub mod gl_loader;
pub mod java_exec_hooks;
pub mod jre_launcher;
pub mod logger;
pub mod lwjgl_dlopen_hook;
pub mod sha1;
pub mod simd_utils;
pub mod stdio_is;
pub mod utils;
pub mod xawt_fake;

use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::{jint, jlong};
use std::ffi::c_char;

// Re-export JNI entry points
pub use environ::Java_com_movtery_zalithlauncher_bridge_ZLBridge_initEnviron;
pub use exit_hook::Java_com_movtery_zalithlauncher_bridge_ZLBridge_initializeGameExitHook;
pub use java_exec_hooks::Java_com_movtery_zalithlauncher_bridge_ZLBridge_hookExec;
pub use lwjgl_dlopen_hook::Java_com_movtery_zalithlauncher_bridge_ZLBridge_installLwjglDlopenHook;

/// Initialize the Rust library
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustInit(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    0
}

/// Get Rust library version
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustGetVersion(
    _env: JNIEnv,
    _class: JClass,
) -> *const i8 {
    "1.0.0-rust\0".as_ptr() as *const i8
}

/// Test function to verify Rust JNI is working
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustTest(
    _env: JNIEnv,
    _class: JClass,
    input: jint,
) -> jint {
    input.wrapping_mul(2)
}

/// Safe wrapper for CPU affinity setting
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustSetBigCoreAffinity(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    match bigcore::set_affinity() {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

/// Safe memory copy for frame buffer operations
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustCopyFrameBuffer(
    _env: JNIEnv,
    _class: JClass,
    src: jlong,
    dst: jlong,
    len: jint,
) -> jint {
    utils::copy_frame_buffer(src as *const u8, dst as *mut u8, len as usize)
        .map(|_| 0)
        .unwrap_or(-1)
}

/// SIMD-accelerated frame buffer copy
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustCopyFrameBufferSimd(
    _env: JNIEnv,
    _class: JClass,
    src: jlong,
    dst: jlong,
    len: jint,
) -> jint {
    let result = unsafe {
        simd_utils::simd_copy_frame_buffer(src as *const u8, dst as *mut u8, len as usize)
    };
    if result >= 0 { 0 } else { -1 }
}

/// Get optimal SIMD buffer size
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustGetSimdOptimalSize(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    simd_utils::get_simd_optimal_size() as jint
}

/// File copy with progress callback
#[no_mangle]
pub unsafe extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustFileCopyWithProgress(
    _env: JNIEnv,
    _class: JClass,
    src_path: jlong,
    dst_path: jlong,
    buffer_size: jint,
    _progress_callback: jlong,
) -> jlong {
    file_io::rust_file_copy_with_progress(
        src_path as *const c_char,
        dst_path as *const c_char,
        buffer_size as usize,
        None,
    )
}

/// Direct file copy
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustFileCopyDirect(
    _env: JNIEnv,
    _class: JClass,
    src_path: jlong,
    dst_path: jlong,
) -> jlong {
    unsafe {
        file_io::rust_file_copy_direct(src_path as *const c_char, dst_path as *const c_char)
    }
}

/// Get file size
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustFileSize(
    _env: JNIEnv,
    _class: JClass,
    path: jlong,
) -> jlong {
    unsafe { file_io::rust_file_size(path as *const c_char) }
}

/// File exists check
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustFileExists(
    _env: JNIEnv,
    _class: JClass,
    path: jlong,
) -> jint {
    unsafe { file_io::rust_file_exists(path as *const c_char) }
}

/// Compute SHA1 hash of a file
/// Returns pointer to hex string (caller must free), or null on error
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustSha1File(
    _env: JNIEnv,
    _class: JClass,
    path: jlong,
) -> *mut c_char {
    let path_ptr = path as *const c_char;
    match sha1::sha1_file_hex(path_ptr) {
        Some(hex) => Box::into_raw(hex.into_boxed_str()) as *mut c_char,
        None => std::ptr::null_mut(),
    }
}

/// Verify file SHA1 against expected hex string
/// Returns 1 if match, 0 if no match, -1 on error
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustSha1Verify(
    _env: JNIEnv,
    _class: JClass,
    path: jlong,
    expected_hex: jlong,
) -> jint {
    let path_ptr = path as *const c_char;
    let hex_ptr = expected_hex as *const c_char;

    if path_ptr.is_null() || hex_ptr.is_null() {
        return -1;
    }

    let expected = unsafe {
        let hex_cstr = std::ffi::CStr::from_ptr(hex_ptr as *const c_char);
        match hex_cstr.to_str() {
            Ok(s) => s,
            Err(_) => return -1,
        }
    };

    if sha1::sha1_verify_file(path_ptr, expected) {
        1
    } else {
        0
    }
}

/// Compute SHA1 hash of bytes
#[no_mangle]
pub unsafe extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustSha1Bytes(
    _env: JNIEnv,
    _class: JClass,
    data: jlong,
    len: jint,
) -> *mut c_char {
    if data == 0 || len <= 0 {
        return std::ptr::null_mut();
    }

    let slice = std::slice::from_raw_parts(data as *const u8, len as usize);
    let hex = sha1::sha1_hex(slice);
    Box::into_raw(hex.into_boxed_str()) as *mut c_char
}