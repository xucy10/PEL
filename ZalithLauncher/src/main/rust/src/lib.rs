//! ZalithLauncher Rust JNI Library
//!
//! Memory-safe implementations of critical native functions.
//! All functions can be called from Java/Kotlin via JNI.

#![allow(dead_code)]

pub mod bigcore;
pub mod awt_bridge;
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
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jbyteArray, jint, jlong, jstring};
use std::ffi::CString;
use std::ptr;

// Re-export JNI entry points
pub use environ::Java_com_movtery_zalithlauncher_bridge_ZLBridge_initEnviron;
pub use exit_hook::Java_com_movtery_zalithlauncher_bridge_ZLBridge_initializeGameExitHook;
pub use java_exec_hooks::Java_com_movtery_zalithlauncher_bridge_ZLBridge_hookExec;
pub use lwjgl_dlopen_hook::Java_com_movtery_zalithlauncher_bridge_ZLBridge_installLwjglDlopenHook;
pub use jre_launcher::Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustLaunchJVM;

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
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    let version = CString::new("1.0.0-rust").unwrap();
    env.new_string(version.to_str().unwrap()).unwrap().into_raw()
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

/// Direct file copy - accepts Java String and returns jlong (bytes copied or -1)
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustFileCopyDirect(
    mut env: JNIEnv,
    _class: JClass,
    src_path: JString,
    dst_path: JString,
) -> jlong {
    let src_str: String = match env.get_string(&src_path) {
        Ok(s) => s.into(),
        Err(_) => return -1,
    };
    let dst_str: String = match env.get_string(&dst_path) {
        Ok(s) => s.into(),
        Err(_) => return -1,
    };

    let src_cstr = match CString::new(src_str.as_str()) {
        Ok(c) => c,
        Err(_) => return -1,
    };
    let dst_cstr = match CString::new(dst_str.as_str()) {
        Ok(c) => c,
        Err(_) => return -1,
    };

    unsafe {
        file_io::rust_file_copy_direct(src_cstr.as_ptr(), dst_cstr.as_ptr())
    }
}

/// File copy with progress callback
#[no_mangle]
pub unsafe extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustFileCopyWithProgress(
    mut env: JNIEnv,
    _class: JClass,
    src_path: JString,
    dst_path: JString,
    buffer_size: jint,
    _progress_callback: jlong,
) -> jlong {
    let src_str: String = match env.get_string(&src_path) {
        Ok(s) => s.into(),
        Err(_) => return -1,
    };
    let dst_str: String = match env.get_string(&dst_path) {
        Ok(s) => s.into(),
        Err(_) => return -1,
    };

    let src_cstr = match CString::new(src_str.as_str()) {
        Ok(c) => c,
        Err(_) => return -1,
    };
    let dst_cstr = match CString::new(dst_str.as_str()) {
        Ok(c) => c,
        Err(_) => return -1,
    };

    file_io::rust_file_copy_with_progress(
        src_cstr.as_ptr(),
        dst_cstr.as_ptr(),
        buffer_size as usize,
        None,
    )
}

/// Get file size - accepts Java String
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustFileSize(
    mut env: JNIEnv,
    _class: JClass,
    path: JString,
) -> jlong {
    let path_str: String = match env.get_string(&path) {
        Ok(s) => s.into(),
        Err(_) => return -1,
    };

    let path_cstr = match CString::new(path_str.as_str()) {
        Ok(c) => c,
        Err(_) => return -1,
    };

    unsafe { file_io::rust_file_size(path_cstr.as_ptr()) }
}

/// File exists check - accepts Java String
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustFileExists(
    mut env: JNIEnv,
    _class: JClass,
    path: JString,
) -> jint {
    let path_str: String = match env.get_string(&path) {
        Ok(s) => s.into(),
        Err(_) => return 0,
    };

    let path_cstr = match CString::new(path_str.as_str()) {
        Ok(c) => c,
        Err(_) => return 0,
    };

    unsafe { file_io::rust_file_exists(path_cstr.as_ptr()) }
}

/// Compute SHA1 hash of a file - accepts Java String
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustSha1File(
    mut env: JNIEnv,
    _class: JClass,
    path: JString,
) -> jstring {
    let path_str: String = match env.get_string(&path) {
        Ok(s) => s.into(),
        Err(_) => return ptr::null_mut(),
    };

    let path_cstr = match CString::new(path_str.as_str()) {
        Ok(c) => c,
        Err(_) => return ptr::null_mut(),
    };

    let hex_opt = sha1::sha1_file_hex(path_cstr.as_ptr());
    match hex_opt {
        Some(hex) => {
            let result = env.new_string(hex.as_str()).unwrap();
            result.into_raw()
        }
        None => ptr::null_mut(),
    }
}

/// Verify file SHA1 against expected hex string - accepts Java Strings
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustSha1Verify(
    mut env: JNIEnv,
    _class: JClass,
    path: JString,
    expected_hex: JString,
) -> jint {
    let path_str: String = match env.get_string(&path) {
        Ok(s) => s.into(),
        Err(_) => return -1,
    };
    let hex_str: String = match env.get_string(&expected_hex) {
        Ok(s) => s.into(),
        Err(_) => return -1,
    };

    let path_cstr = match CString::new(path_str.as_str()) {
        Ok(c) => c,
        Err(_) => return -1,
    };

    if sha1::sha1_verify_file(path_cstr.as_ptr(), hex_str.as_str()) {
        1
    } else {
        0
    }
}

/// Compute SHA1 hash of bytes - accepts Java byte array
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustSha1Bytes(
    env: JNIEnv,
    _class: JClass,
    data: jbyteArray,
) -> jstring {
    let java_array = unsafe { jni::objects::JByteArray::from_raw(data) };
    let java_bytes: Vec<u8> = match env.convert_byte_array(&java_array) {
        Ok(b) => b,
        Err(_) => return ptr::null_mut(),
    };

    let hex = sha1::sha1_hex(&java_bytes);
    let result = env.new_string(hex.as_str()).unwrap();
    result.into_raw()
}

/// Create directory recursively - accepts Java String
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustCreateDirs(
    mut env: JNIEnv,
    _class: JClass,
    path: JString,
) -> jint {
    let path_str: String = match env.get_string(&path) {
        Ok(s) => s.into(),
        Err(_) => return -1,
    };

    let path_cstr = match CString::new(path_str.as_str()) {
        Ok(c) => c,
        Err(_) => return -1,
    };

    unsafe { file_io::rust_create_dirs(path_cstr.as_ptr()) }
}

/// Delete file - accepts Java String
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustDeleteFile(
    mut env: JNIEnv,
    _class: JClass,
    path: JString,
) -> jint {
    let path_str: String = match env.get_string(&path) {
        Ok(s) => s.into(),
        Err(_) => return -1,
    };

    let path_cstr = match CString::new(path_str.as_str()) {
        Ok(c) => c,
        Err(_) => return -1,
    };

    unsafe { file_io::rust_delete_file(path_cstr.as_ptr()) }
}

/// dlopen - Load a dynamic library using Rust libc
/// Returns true (1) on success, false (0) on failure
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustDlopen(
    mut env: JNIEnv,
    _class: JClass,
    name: JString,
) -> jboolean {
    let name_str: String = match env.get_string(&name) {
        Ok(s) => s.into(),
        Err(_) => return 0,
    };

    let name_cstr = match CString::new(name_str.as_str()) {
        Ok(c) => c,
        Err(_) => return 0,
    };

    unsafe {
        let handle = libc::dlopen(name_cstr.as_ptr(), libc::RTLD_GLOBAL | libc::RTLD_LAZY);
        if handle.is_null() {
            let error = libc::dlerror();
            if !error.is_null() {
                let _error_str = std::ffi::CStr::from_ptr(error).to_string_lossy();
                eprintln!("Rust dlopen failed");
            }
            0
        } else {
            1
        }
    }
}

/// chdir - Change current working directory using Rust libc
/// Returns 0 on success, -1 on failure
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustChdir(
    mut env: JNIEnv,
    _class: JClass,
    path: JString,
) -> jint {
    let path_str: String = match env.get_string(&path) {
        Ok(s) => s.into(),
        Err(_) => return -1,
    };

    let path_cstr = match CString::new(path_str.as_str()) {
        Ok(c) => c,
        Err(_) => return -1,
    };

    unsafe {
        libc::chdir(path_cstr.as_ptr())
    }
}

/// Get current working directory
/// Returns the current working directory as a Java String
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustGetcwd(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    let mut buf = [0u8; libc::PATH_MAX as usize];
    unsafe {
        if libc::getcwd(buf.as_mut_ptr() as *mut libc::c_char, libc::PATH_MAX as libc::size_t).is_null() {
            return ptr::null_mut();
        }
    }
    let cwd = std::ffi::CStr::from_bytes_until_nul(&buf).unwrap().to_string_lossy();
    env.new_string(cwd.as_ref()).unwrap().into_raw()
}

/// Set LD_LIBRARY_PATH environment variable
/// This is Android-specific and calls android_update_LD_LIBRARY_PATH
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustSetLdLibraryPath(
    mut env: JNIEnv,
    _class: JClass,
    ld_library_path: JString,
) -> jint {
    let path_str: String = match env.get_string(&ld_library_path) {
        Ok(s) => s.into(),
        Err(_) => return -1,
    };

    let path_cstr = match CString::new(path_str.as_str()) {
        Ok(c) => c,
        Err(_) => return -1,
    };

    unsafe {
        let libdl_name = CString::new("libdl.so").unwrap();
        let libdl_handle = libc::dlopen(libdl_name.as_ptr(), libc::RTLD_LAZY);
        if libdl_handle.is_null() {
            eprintln!("Rust: Failed to open libdl.so");
            return -1;
        }

        let func_names = [
            b"android_update_LD_LIBRARY_PATH\0".as_ptr() as *const libc::c_char,
            b"__loader_android_update_LD_LIBRARY_PATH\0".as_ptr() as *const libc::c_char,
        ];

        let mut func: *mut libc::c_void = ptr::null_mut();
        for name in func_names.iter() {
            func = libc::dlsym(libdl_handle, *name);
            if !func.is_null() {
                break;
            }
        }

        if func.is_null() {
            eprintln!("Rust: Failed to find android_update_LD_LIBRARY_PATH");
            libc::dlclose(libdl_handle);
            return -1;
        }

        // Set LD_LIBRARY_PATH env var first
        let env_name = CString::new("LD_LIBRARY_PATH").unwrap();
        libc::setenv(env_name.as_ptr(), path_cstr.as_ptr(), 1);

        // Call the function
        type UpdateFunc = extern "C" fn(*const libc::c_char);
        let update_func: UpdateFunc = std::mem::transmute(func);
        update_func(path_cstr.as_ptr());

        libc::dlclose(libdl_handle);
        0
    }
}