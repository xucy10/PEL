//! Global environment management - Rust implementation
//!
//! Manages the global pojav_environ structure for JNI communication.

use std::ffi::CString;
use std::sync::atomic::{AtomicPtr, Ordering};

/// Global environment pointer - atomic for thread safety
static POJAV_ENVIRON: AtomicPtr<std::ffi::c_void> = AtomicPtr::new(std::ptr::null_mut());

/// Initialize the environment (called from JNI OnLoad)
#[no_mangle]
pub extern "C" fn env_init() {
    unsafe {
        let env_ptr = POJAV_ENVIRON.load(Ordering::SeqCst);

        if env_ptr.is_null() {
            // Allocate new environment
            let new_env = libc::malloc(1024); // Placeholder size
            if new_env.is_null() {
                libc::abort();
            }

            // Zero initialize
            libc::memset(new_env, 0, 1024);

            // Store pointer as hex string in environment variable
            let env_str = format!("{:p}", new_env);
            if let Ok(env_cstr) = CString::new(env_str) {
                libc::setenv(env_cstr.as_ptr(), env_cstr.as_ptr(), 1);
            }

            POJAV_ENVIRON.store(new_env, Ordering::SeqCst);
            println!("Environ: Created new environ at {:?}", new_env);
        } else {
            println!("Environ: Found existing environ at {:?}", env_ptr);
        }
    }
}

/// Get the global environ pointer
#[no_mangle]
pub extern "C" fn get_pojav_environ() -> *mut std::ffi::c_void {
    POJAV_ENVIRON.load(Ordering::SeqCst)
}

/// JNI initialization function
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_initEnviron(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
) {
    env_init();
}