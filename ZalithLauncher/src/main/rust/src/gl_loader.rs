//! OpenGL loader - Rust implementation
//!
//! Loads OpenGL/EGL function pointers via dynamic linking.

use libc::{c_char, dlopen, dlsym, dlerror};
use std::ffi::CStr;

/// Type for eglGetProcAddress
type EglGetProcAddressFn = Option<extern "C" fn(*const c_char) -> *mut libc::c_void>;
/// Type for OSMesaGetProcAddress
type OsmGetProcAddressFn = Option<extern "C" fn(*const c_char) -> *mut libc::c_void>;

/// Global function pointers
static mut EGL_GET_PROC_ADDRESS: EglGetProcAddressFn = None;
static mut OSMESA_GET_PROC_ADDRESS: OsmGetProcAddressFn = None;

/// Load a symbol from a handle
fn load_symbol(handle: *mut libc::c_void, symbol_name: &CStr) -> *mut libc::c_void {
    unsafe {
        let symbol = dlsym(handle, symbol_name.as_ptr());
        if symbol.is_null() {
            let err_ptr = dlerror();
            if !err_ptr.is_null() {
                let err_str = CStr::from_ptr(err_ptr).to_string_lossy();
                eprintln!("Error[Load Symbol]: Failed to load symbol '{}': {}", symbol_name.to_string_lossy(), err_str);
            }
        }
        symbol
    }
}

/// Load OSMesaGetProcAddress and look up a symbol
#[no_mangle]
pub unsafe extern "C" fn OSMGetProcAddress(symbol_name: *const c_char) -> *mut libc::c_void {
    if symbol_name.is_null() {
        return std::ptr::null_mut();
    }

    let name = CStr::from_ptr(symbol_name);

    // Load the library if not already loaded (using egl handle for OSMesa)
    static mut HANDLE: *mut libc::c_void = std::ptr::null_mut();
    if HANDLE.is_null() {
        HANDLE = dlopen(b"libEGL.so\0".as_ptr() as *const c_char, libc::RTLD_NOLOAD);
        if !HANDLE.is_null() {
            let osm_sym = CStr::from_bytes_with_nul(b"OSMesaGetProcAddress\0").unwrap();
            OSMESA_GET_PROC_ADDRESS = Some(std::mem::transmute(dlsym(HANDLE, osm_sym.as_ptr())));
        }
    }

    // Try OSMesaGetProcAddress first
    if let Some(osm_get_proc) = OSMESA_GET_PROC_ADDRESS {
        let result = osm_get_proc(symbol_name);
        if !result.is_null() {
            return result;
        }
    }

    // Fall back to direct symbol lookup
    if !HANDLE.is_null() {
        load_symbol(HANDLE, name)
    } else {
        std::ptr::null_mut()
    }
}

/// Load eglGetProcAddress and look up a symbol
#[no_mangle]
pub unsafe extern "C" fn GLGetProcAddress(symbol_name: *const c_char) -> *mut libc::c_void {
    if symbol_name.is_null() {
        return std::ptr::null_mut();
    }

    let name = CStr::from_ptr(symbol_name);

    // Load the library if not already loaded
    static mut HANDLE: *mut libc::c_void = std::ptr::null_mut();
    if HANDLE.is_null() {
        HANDLE = dlopen(b"libEGL.so\0".as_ptr() as *const c_char, libc::RTLD_NOLOAD);
        if !HANDLE.is_null() {
            let egl_sym = CStr::from_bytes_with_nul(b"eglGetProcAddress\0").unwrap();
            EGL_GET_PROC_ADDRESS = Some(std::mem::transmute(dlsym(HANDLE, egl_sym.as_ptr())));
        }
    }

    // Try eglGetProcAddress first
    if let Some(egl_get_proc) = EGL_GET_PROC_ADDRESS {
        let result = egl_get_proc(symbol_name);
        if !result.is_null() {
            return result;
        }
    }

    // Fall back to direct symbol lookup
    if !HANDLE.is_null() {
        load_symbol(HANDLE, name)
    } else {
        std::ptr::null_mut()
    }
}