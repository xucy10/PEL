//! Stdio logging bridge - stub implementation
//! Full implementation requires complex pipe/thread handling

use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::jint;

/// Start logging (stub)
#[no_mangle]
pub unsafe extern "C" fn Java_com_movtery_zalithlauncher_bridge_LoggerBridge_start(
    _env: JNIEnv,
    _class: JObject,
    _log_path: JString,
) -> jint {
    0
}

/// Append to log (stub)
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_LoggerBridge_append(
    _env: JNIEnv,
    _class: JObject,
    _text: JString,
) -> jint {
    0
}

/// Set the log listener (stub)
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_LoggerBridge_setListener(
    _env: JNIEnv,
    _class: JObject,
    _log_listener: JObject,
) -> jint {
    0
}

/// Setup exit method (stub)
#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_setupExitMethod(
    _env: JNIEnv,
    _class: JObject,
    _context: JObject,
) -> jint {
    0
}