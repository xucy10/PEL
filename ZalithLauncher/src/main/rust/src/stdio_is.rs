//! Stdio redirection and logging - Rust implementation
//!
//! Provides stdout/stderr capture for game logging.

use std::io::Write;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};

use jni::objects::{JClass, JString};
use jni::sys::jint;

/// Logger state
static LOGGER_ACTIVE: AtomicBool = AtomicBool::new(false);

/// Log file descriptor
static LOG_FD: AtomicIsize = AtomicIsize::new(-1);

/// Initialize logging to a file path
#[no_mangle]
pub unsafe extern "C" fn Java_com_movtery_zalithlauncher_bridge_LoggerBridge_start(
    mut env: jni::JNIEnv,
    _class: JClass,
    log_path: JString,
) -> jint {
    let path = match env.get_string(&log_path) {
        Ok(p) => p,
        Err(_) => return -1,
    };
    let path_str = path.to_str().unwrap_or("/dev/null");

    // Open log file
    let fd = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path_str)
        .map(|f| f.as_raw_fd() as isize)
        .unwrap_or(-1);

    if fd < 0 {
        return -1;
    }

    LOG_FD.store(fd, Ordering::SeqCst);
    LOGGER_ACTIVE.store(true, Ordering::SeqCst);

    0
}

/// Append text to log
#[no_mangle]
pub unsafe extern "C" fn Java_com_movtery_zalithlauncher_bridge_LoggerBridge_append(
    mut env: jni::JNIEnv,
    _class: JClass,
    text: JString,
) -> jint {
    let msg = match env.get_string(&text) {
        Ok(m) => m,
        Err(_) => return -1,
    };
    let msg_bytes = msg.to_bytes();

    let fd = LOG_FD.load(Ordering::SeqCst);
    if fd >= 0 {
        let mut file = unsafe { std::fs::File::from_raw_fd(fd as i32) };
        let _ = file.write_all(msg_bytes);
        let _ = file.write_all(b"\n");
        let _ = file.sync_all();
    }

    0
}

/// Set log listener (no-op for now)
#[no_mangle]
pub unsafe extern "C" fn Java_com_movtery_zalithlauncher_bridge_LoggerBridge_setListener(
    _env: jni::JNIEnv,
    _class: JClass,
    _listener: jni::objects::JObject,
) -> jint {
    0
}

/// Setup exit handler (no-op for now)
#[no_mangle]
pub unsafe extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_setupExitMethod(
    _env: jni::JNIEnv,
    _class: JClass,
    _context: jni::objects::JObject,
) -> jint {
    0
}
