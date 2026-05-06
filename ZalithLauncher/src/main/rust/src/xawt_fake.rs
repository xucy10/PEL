//! AWT X11 fake implementation - Rust version
//!
//! Empty JNI stubs for AWT functionality that don't need real implementation.

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jint};

/// Initialize AWT event IDs
#[no_mangle]
pub unsafe extern "C" fn Java_java_awt_AWTEvent_initIDs(
    _env: JNIEnv,
    _cls: JClass,
) {
}

/// Initialize Button IDs
#[no_mangle]
pub unsafe extern "C" fn Java_java_awt_Button_initIDs(
    _env: JNIEnv,
    _cls: JClass,
) {
}

/// Initialize Component IDs
#[no_mangle]
pub unsafe extern "C" fn Java_java_awt_Component_initIDs(
    _env: JNIEnv,
    _cls: JClass,
) {
}

/// Initialize Container IDs
#[no_mangle]
pub unsafe extern "C" fn Java_java_awt_Container_initIDs(
    _env: JNIEnv,
    _cls: JClass,
) {
}

/// Initialize Checkbox IDs
#[no_mangle]
pub unsafe extern "C" fn Java_java_awt_Checkbox_initIDs(
    _env: JNIEnv,
    _cls: JClass,
) {
}

/// Initialize Cursor IDs
#[no_mangle]
pub unsafe extern "C" fn Java_java_awt_Cursor_initIDs(
    _env: JNIEnv,
    _cls: JClass,
) {
}

/// Initialize Dialog IDs
#[no_mangle]
pub unsafe extern "C" fn Java_java_awt_Dialog_initIDs(
    _env: JNIEnv,
    _cls: JClass,
) {
}

/// Initialize Frame IDs
#[no_mangle]
pub unsafe extern "C" fn Java_java_awt_Frame_initIDs(
    _env: JNIEnv,
    _cls: JClass,
) {
}

/// Initialize Window IDs
#[no_mangle]
pub unsafe extern "C" fn Java_java_awt_Window_initIDs(
    _env: JNIEnv,
    _cls: JClass,
) {
}

/// Initialize Insets IDs
#[no_mangle]
pub unsafe extern "C" fn Java_java_awt_Insets_initIDs(
    _env: JNIEnv,
    _cls: JClass,
) {
}

/// Initialize Menu IDs
#[no_mangle]
pub unsafe extern "C" fn Java_java_awt_Menu_initIDs(
    _env: JNIEnv,
    _cls: JClass,
) {
}

/// Initialize MenuItem IDs
#[no_mangle]
pub unsafe extern "C" fn Java_java_awt_MenuItem_initIDs(
    _env: JNIEnv,
    _cls: JClass,
) {
}

/// Close splash screen - no-op
#[no_mangle]
pub unsafe extern "C" fn Java_sun_awt_SunToolkit_closeSplashScreen(
    _env: JNIEnv,
    _cls: JClass,
) {
}

/// Check GTK - always returns false
#[no_mangle]
pub unsafe extern "C" fn Java_sun_awt_UNIXToolkit_check_1gtk(
    _env: JNIEnv,
    _klass: JClass,
    _version: jint,
) -> jboolean {
    jboolean::from(false)
}

/// Get GTK version - returns 1
#[no_mangle]
pub unsafe extern "C" fn Java_sun_awt_UNIXToolkit_get_1gtk_1version(
    _env: JNIEnv,
    _klass: JClass,
) -> jint {
    1
}

/// GTK version check - always returns false
#[no_mangle]
pub unsafe extern "C" fn Java_sun_awt_UNIXToolkit_gtkCheckVersionImpl(
    _env: JNIEnv,
    _this: JObject,
    _major: jint,
    _minor: jint,
    _micro: jint,
) -> jboolean {
    jboolean::from(false)
}

/// Load GTK - always returns false
#[no_mangle]
pub unsafe extern "C" fn Java_sun_awt_UNIXToolkit_load_1gtk(
    _env: JNIEnv,
    _klass: JClass,
    _version: jint,
    _verbose: jboolean,
) -> jboolean {
    jboolean::from(false)
}

/// Load GTK icon - always returns false
#[no_mangle]
pub unsafe extern "C" fn Java_sun_awt_UNIXToolkit_load_1gtk_1icon(
    _env: JNIEnv,
    _this: JObject,
    _filename: JString,
) -> jboolean {
    jboolean::from(false)
}

/// Native sync - no-op
#[no_mangle]
pub unsafe extern "C" fn Java_sun_awt_UNIXToolkit_nativeSync(
    _env: JNIEnv,
    _this: JObject,
) {
}

/// Unload GTK - always returns false
#[no_mangle]
pub unsafe extern "C" fn Java_sun_awt_UNIXToolkit_unload_1gtk(
    _env: JNIEnv,
    _klass: JClass,
) -> jboolean {
    jboolean::from(false)
}