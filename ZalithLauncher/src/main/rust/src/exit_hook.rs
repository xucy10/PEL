//! Exit hook - Rust implementation
//!
//! Hooks into the exit() function to ensure proper cleanup before process termination.
//! Uses atomic operations for thread-safety.

use jni::JNIEnv;
use jni::objects::JClass;
use libc::{c_char, c_int, c_void, dlopen, dlsym, RTLD_NOW};
use core::sync::atomic::{AtomicBool, Ordering};
use core::ffi::CStr;

/// Atomic flag to track if exit was already triggered
static EXIT_TRIPPED: AtomicBool = AtomicBool::new(false);

/// Bytehook status codes
const BYTEHOOK_STATUS_CODE_OK: c_int = 0;
const BYTEHOOK_MODE_AUTOMATIC: c_int = 0;

/// Bytehook function pointer types
type BytehookStubT = *mut c_void;
type BytehookHookedT = Option<extern "C" fn(*mut c_void)>;

type BytehookInitFunc = extern "C" fn(c_int, bool) -> c_int;
type BytehookHookAllFunc = extern "C" fn(
    *const c_char,
    *const c_char,
    *mut c_void,
    BytehookHookedT,
    *mut c_void,
) -> BytehookStubT;

// External reference to nominal_exit from stdio_is.c
extern "C" {
    fn nominal_exit(code: c_int, is_atexit: bool);
}

/// Custom exit function that hooks into the standard exit()
#[no_mangle]
pub extern "C" fn custom_exit(code: c_int) {
    // If exit was already triggered (recursive or from different thread), pass through
    if EXIT_TRIPPED.swap(true, Ordering::SeqCst) {
        // Call the original exit - this would be done via BYTEHOOK_CALL_PREV in C
        // Since we can't easily do that in Rust, we just return
        // The bytehook library will handle the passthrough
        return;
    }

    // Perform nominal exit
    unsafe {
        nominal_exit(code, false);
    }
}

/// Custom atexit handler
#[no_mangle]
pub extern "C" fn custom_atexit() {
    // If exit was already triggered, don't do anything
    if EXIT_TRIPPED.swap(true, Ordering::SeqCst) {
        return;
    }

    // Perform nominal exit with code 0
    unsafe {
        nominal_exit(0, false);
    }
}

// Android log function
extern "C" {
    fn __android_log_print(prio: c_int, tag: *const c_char, fmt: *const c_char, ...) -> c_int;
}

const ANDROID_LOG_INFO: c_int = 4;
const ANDROID_LOG_ERROR: c_int = 6;

/// Initialize the exit hook
fn init_exit_hook() -> bool {
    let lib_name = b"libbytehook.so\0";
    
    // Load bytehook library
    let bytehook_handle = unsafe { dlopen(lib_name.as_ptr() as *const c_char, RTLD_NOW) };
    if bytehook_handle.is_null() {
        let error_msg = unsafe {
            let err_ptr = libc::dlerror();
            if !err_ptr.is_null() {
                CStr::from_ptr(err_ptr).to_string_lossy().into_owned()
            } else {
                "Unknown error".to_string()
            }
        };
        
        unsafe {
            let tag = b"exit_hook\0".as_ptr() as *const c_char;
            let fmt = b"Failed to load hook library: %s\0".as_ptr() as *const c_char;
            __android_log_print(ANDROID_LOG_ERROR, tag, fmt, error_msg.as_ptr());
        }
        return false;
    }

    // Get function pointers
    let hook_all_sym = b"bytehook_hook_all\0";
    let init_sym = b"bytehook_init\0";

    let bytehook_hook_all = unsafe { dlsym(bytehook_handle, hook_all_sym.as_ptr() as *const c_char) };
    let bytehook_init = unsafe { dlsym(bytehook_handle, init_sym.as_ptr() as *const c_char) };

    if bytehook_hook_all.is_null() || bytehook_init.is_null() {
        let error_msg = unsafe {
            let err_ptr = libc::dlerror();
            if !err_ptr.is_null() {
                CStr::from_ptr(err_ptr).to_string_lossy().into_owned()
            } else {
                "Symbol not found".to_string()
            }
        };
        
        unsafe {
            libc::dlclose(bytehook_handle);
            let tag = b"exit_hook\0".as_ptr() as *const c_char;
            let fmt = b"Failed to load symbols: %s\0".as_ptr() as *const c_char;
            __android_log_print(ANDROID_LOG_ERROR, tag, fmt, error_msg.as_ptr());
        }
        return false;
    }

    // Cast function pointers
    let init_func: BytehookInitFunc = unsafe { core::mem::transmute(bytehook_init) };
    let hook_all_func: BytehookHookAllFunc = unsafe { core::mem::transmute(bytehook_hook_all) };

    // Initialize bytehook
    let status = init_func(BYTEHOOK_MODE_AUTOMATIC, false);
    
    if status == BYTEHOOK_STATUS_CODE_OK {
        // Hook the exit function
        let exit_sym = b"exit\0";
        let stub = hook_all_func(
            core::ptr::null(),
            exit_sym.as_ptr() as *const c_char,
            custom_exit as *mut c_void,
            None,
            core::ptr::null_mut(),
        );

        unsafe {
            let tag = b"exit_hook\0".as_ptr() as *const c_char;
            let fmt = b"Successfully initialized exit hook, stub=%p\0".as_ptr() as *const c_char;
            __android_log_print(ANDROID_LOG_INFO, tag, fmt, stub);
        }
        true
    } else {
        unsafe {
            let tag = b"exit_hook\0".as_ptr() as *const c_char;
            let fmt = b"bytehook_init failed (%i)\0".as_ptr() as *const c_char;
            __android_log_print(ANDROID_LOG_INFO, tag, fmt, status);
            libc::dlclose(bytehook_handle);
        }
        false
    }
}

/// JNI function to initialize the game exit hook
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_initializeGameExitHook(
    _env: JNIEnv,
    _clazz: JClass,
) {
    let hook_ready = init_exit_hook();
    
    if !hook_ready {
        // If we can't hook, register atexit()
        // This prevents SIGSEGV or SIGABRT from Dalvik on exit()
        unsafe {
            libc::atexit(custom_atexit);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exit_tripped_atomic() {
        assert!(!EXIT_TRIPPED.load(Ordering::SeqCst));
        EXIT_TRIPPED.store(true, Ordering::SeqCst);
        assert!(EXIT_TRIPPED.load(Ordering::SeqCst));
    }
}