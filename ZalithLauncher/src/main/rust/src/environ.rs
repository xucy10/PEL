//! Environment variable handling

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jint, jstring};
use std::ffi::CString;
use std::ptr;

/// Hook for Java_exec_hooks
#[no_mangle]
pub unsafe extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_initEnviron(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    0
}