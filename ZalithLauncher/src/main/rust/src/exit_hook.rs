//! Exit hook stub

use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jint;

#[no_mangle]
pub unsafe extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_initializeGameExitHook(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    0
}