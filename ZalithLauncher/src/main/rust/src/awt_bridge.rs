//! AWT Bridge - stub implementation
//! Full implementation requires complex JNI thread attachment handling

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jint, jintArray};
use std::ptr;

#[no_mangle]
pub unsafe extern "C" fn JNI_OnLoad(vm: *mut jni::JavaVM, _reserved: *mut libc::c_void) -> jint {
    jni::sys::JNI_VERSION_1_4
}

#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_sendInputData(
    _env: JNIEnv,
    _class: JObject,
    _type_val: jint,
    _i1: jint,
    _i2: jint,
    _i3: jint,
    _i4: jint,
) {
}

#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_renderAWTScreenFrame(
    _env: JNIEnv,
    _class: JObject,
) -> jintArray {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn Java_net_java_openjdk_cacio_ctc_CTCClipboard_nQuerySystemClipboard(
    _env: JNIEnv,
    _clazz: JClass,
) {
}

#[no_mangle]
pub extern "C" fn Java_net_java_openjdk_cacio_ctc_CTCClipboard_nPutClipboardData(
    _env: JNIEnv,
    _clazz: JClass,
    _clipboard_data: JString,
    _clipboard_data_mime: JString,
) {
}

#[no_mangle]
pub extern "C" fn Java_net_java_openjdk_cacio_ctc_CTCDesktopPeer_openFile(
    _env: JNIEnv,
    _clazz: JClass,
    _file_path: JString,
) {
}

#[no_mangle]
pub extern "C" fn Java_net_java_openjdk_cacio_ctc_CTCDesktopPeer_openUri(
    _env: JNIEnv,
    _clazz: JClass,
    _uri: JString,
) {
}

#[no_mangle]
pub extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_moveWindow(
    _env: JNIEnv,
    _class: JObject,
    _xoff: jint,
    _yoff: jint,
) {
}