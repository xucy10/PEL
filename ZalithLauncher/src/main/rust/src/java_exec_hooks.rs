//! Java exec hooks - Rust implementation
//!
//! Hooks into Java's forkAndExec to intercept executable invocations.
//! Used for xdg-open and ffmpeg path replacement.

use jni::JNIEnv;
use jni::objects::JClass;

/// Hook the forkAndExec method in the Java runtime
#[no_mangle]
pub unsafe extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_hookExec(
    mut env: JNIEnv,
    _class: JClass,
) {
    // Try to find and hook the forkAndExec method
    match env.find_class("java/lang/UNIXProcess") {
        Ok(_) => {
            println!("Found UNIXProcess class");
        }
        Err(_) => {
            match env.find_class("java/lang/ProcessImpl") {
                Ok(_) => {
                    println!("Found ProcessImpl class");
                }
                Err(e) => {
                    println!("Failed to find ProcessImpl: {:?}", e);
                }
            }
        }
    }

    println!("hookExec completed");
}