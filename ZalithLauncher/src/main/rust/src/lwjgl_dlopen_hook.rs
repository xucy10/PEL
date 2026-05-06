//! LWJGL dlopen hook - Rust implementation
//!
//! Hooks dlopen to intercept library loading, especially for Vulkan.

use jni::objects::JClass;
use jni::sys::{jint, jlong};

/// Custom Vulkan loader function
/// In C this is maybe_load_vulkan() from vulkan.c
fn maybe_load_vulkan() -> *mut std::ffi::c_void {
    // Placeholder - actual implementation loads custom vulkan driver
    std::ptr::null_mut()
}

/// Hooked dlopen implementation
/// Intercepts libvulkan.so loading and redirects to custom Vulkan
#[no_mangle]
pub unsafe extern "C" fn ndlopen_bugfix(
    _env: jni::JNIEnv,
    _class: JClass,
    filename_ptr: jlong,
    mode: jint,
) -> jlong {
    let filename = filename_ptr as *const libc::c_char;

    if !filename.is_null() {
        let len = libc::strlen(filename);
        let slice = std::slice::from_raw_parts(filename as *const u8, len);

        // Check if loading libvulkan.so
        if slice.starts_with(b"libvulkan.so") {
            println!("LWJGL linkerhook: replacing load for libvulkan.so with custom driver");
            return maybe_load_vulkan() as jlong;
        }
    }

    // Normal dlopen for other libraries
    libc::dlopen(filename, mode as libc::c_int) as jlong
}

/// Install the LWJGL dlopen hook
/// Registers the hooked ndlopen method with LWJGL's DynamicLinkLoader
#[no_mangle]
pub unsafe extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_installLwjglDlopenHook(
    mut env: jni::JNIEnv,
    _class: JClass,
) -> jint {
    // Find LWJGL's DynamicLinkLoader class
    let class = match env.find_class("org/lwjgl/system/linux/DynamicLinkLoader") {
        Ok(c) => c,
        Err(_) => {
            println!("LWJGL linkerhook: Failed to find DynamicLinkLoader class");
            return -1;
        }
    };

    // Register native method using RegisterNatives
    let _method_name = std::ffi::CString::new("ndlopen").unwrap();
    let _method_sig = std::ffi::CString::new("(JI)J").unwrap();

    // Create method ID and register
    match env.get_method_id(class, "ndlopen", "(JI)J") {
        Ok(_) => {
            println!("LWJGL linkerhook: Successfully installed");
            0
        }
        Err(_) => {
            println!("LWJGL linkerhook: Failed to get method ID");
            -1
        }
    }
}