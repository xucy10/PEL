//! Environment variable handling

use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jint;

/// Hook for Java_exec_hooks
#[no_mangle]
pub unsafe extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_initEnviron(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    0
}