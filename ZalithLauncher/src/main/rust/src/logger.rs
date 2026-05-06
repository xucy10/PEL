//! Logging utilities - Rust implementation
//!
//! Simple logging functions for the launcher.

use std::ffi::CStr;
use libc::c_char;

/// Log a message with the given level
#[no_mangle]
pub unsafe extern "C" fn zl_log(level: *const c_char, fmt: *const c_char) -> () {
    if level.is_null() || fmt.is_null() {
        return;
    }

    let level_str = CStr::from_ptr(level).to_string_lossy();
    let fmt_str = CStr::from_ptr(fmt).to_string_lossy();

    eprintln!("[{}] {}", level_str, fmt_str);
}

/// Log with single argument
#[no_mangle]
pub unsafe extern "C" fn zl_log1(level: *const c_char, fmt: *const c_char, arg1: *const c_char) -> () {
    if level.is_null() || fmt.is_null() || arg1.is_null() {
        return;
    }

    let level_str = CStr::from_ptr(level).to_string_lossy();
    let fmt_str = CStr::from_ptr(fmt).to_string_lossy();
    let arg_str = CStr::from_ptr(arg1).to_string_lossy();

    eprintln!("[{}] {}", level_str, fmt_str.replace("{}", &arg_str));
}