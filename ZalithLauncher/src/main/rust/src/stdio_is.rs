//! Stdio_is stub

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jint;

#[no_mangle]
pub unsafe extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_isatty(
    _env: JNIEnv,
    _class: JClass,
    _fd: jint,
) -> jint {
    0
}