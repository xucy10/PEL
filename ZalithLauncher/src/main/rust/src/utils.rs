//! Utility functions - stub implementation

use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::{jboolean, jint, jlong, jstring};
use std::ffi::CString;
use std::ptr;

/// Set LD_LIBRARY_PATH using Android's update function
#[no_mangle]
pub unsafe extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_setLdLibraryPath(
    mut env: JNIEnv,
    _class: JObject,
    ld_library_path: JString,
) -> jint {
    let path_str = match env.get_string(&ld_library_path) {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let path_cstr = match CString::new(path_str.to_string_lossy().as_bytes()) {
        Ok(c) => c,
        Err(_) => return -1,
    };

    let libdl_name = CString::new("libdl.so").unwrap();
    let libdl_handle = libc::dlopen(libdl_name.as_ptr(), libc::RTLD_LAZY);
    if libdl_handle.is_null() {
        eprintln!("Rust: Failed to open libdl.so");
        return -1;
    }

    // Try both possible function names
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
        libc::dlclose(libdl_handle);
        return -1;
    }

    // Call the update function
    type UpdateFunc = extern "C" fn(*const libc::c_char);
    let update_func: UpdateFunc = std::mem::transmute(func);
    update_func(path_cstr.as_ptr());

    libc::dlclose(libdl_handle);
    0
}

/// dlopen wrapper
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_dlopen(
    mut env: JNIEnv,
    _class: JObject,
    name: JString,
) -> jboolean {
    let name_str = match env.get_string(&name) {
        Ok(s) => s,
        Err(_) => return 0,
    };

    let name_cstr = match CString::new(name_str.to_string_lossy().as_bytes()) {
        Ok(c) => c,
        Err(_) => return 0,
    };

    unsafe {
        let handle = libc::dlopen(name_cstr.as_ptr(), libc::RTLD_GLOBAL | libc::RTLD_LAZY);
        if handle.is_null() {
            0
        } else {
            1
        }
    }
}

/// chdir wrapper
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_chdir(
    mut env: JNIEnv,
    _class: JObject,
    name: JString,
) -> jint {
    let name_str = match env.get_string(&name) {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let name_cstr = match CString::new(name_str.to_string_lossy().as_bytes()) {
        Ok(c) => c,
        Err(_) => return -1,
    };

    unsafe { libc::chdir(name_cstr.as_ptr()) }
}

/// Get AWT surface (stub)
#[no_mangle]
pub extern "C" fn Java_android_view_Surface_nativeGetBridgeSurfaceAWT(
    _env: JNIEnv,
    _class: JObject,
) -> jlong {
    0
}

/// Register natives (stub)
#[no_mangle]
pub extern "C" fn Java_android_os_OpenJDKNativeRegister_nativeRegisterNatives(
    _env: JNIEnv,
    _class: JObject,
    _register_symbol: JString,
) -> jint {
    0
}

/// Safe memory copy for frame buffer
#[inline(always)]
pub fn copy_frame_buffer(src: *const u8, dst: *mut u8, len: usize) -> Result<(), i32> {
    unsafe {
        std::ptr::copy_nonoverlapping(src, dst, len);
    }
    Ok(())
}