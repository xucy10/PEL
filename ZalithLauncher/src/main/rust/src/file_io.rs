//! File I/O operations with Rust safety guarantees
//!
//! Safe wrappers around libc file operations.

use libc::{c_char, c_int, size_t};

// open flags
const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_CREAT: c_int = 64;
const O_TRUNC: c_int = 512;

/// Direct file copy without progress callback
#[no_mangle]
pub unsafe extern "C" fn rust_file_copy_direct(
    src: *const c_char,
    dst: *const c_char,
) -> i64 {
    let src_fd = libc::open(src, O_RDONLY, 0o644);
    if src_fd < 0 {
        return -1;
    }

    let dst_fd = libc::open(dst, O_WRONLY | O_CREAT | O_TRUNC, 0o644);
    if dst_fd < 0 {
        libc::close(src_fd);
        return -1;
    }

    let mut buf = [0u8; 65536];
    let mut total: i64 = 0;

    loop {
        let n = libc::read(src_fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len() as size_t);
        if n < 0 {
            libc::close(src_fd);
            libc::close(dst_fd);
            return -1;
        }
        if n == 0 {
            break;
        }
        let mut written = 0i64;
        while written < n as i64 {
            let w = libc::write(dst_fd, buf.as_mut_ptr().offset(written as isize) as *mut libc::c_void, (n as i64 - written) as size_t);
            if w < 0 {
                libc::close(src_fd);
                libc::close(dst_fd);
                return -1;
            }
            written += w as i64;
        }
        total += n as i64;
    }

    libc::close(src_fd);
    libc::close(dst_fd);
    total
}

/// Direct file copy with progress callback (callback not yet implemented)
#[no_mangle]
pub unsafe extern "C" fn rust_file_copy_with_progress(
    src: *const c_char,
    dst: *const c_char,
    buffer_size: size_t,
    _progress_callback: Option<extern "C" fn(i64, i64)>,
) -> i64 {
    let src_fd = libc::open(src, O_RDONLY, 0o644);
    if src_fd < 0 {
        return -1;
    }

    let dst_fd = libc::open(dst, O_WRONLY | O_CREAT | O_TRUNC, 0o644);
    if dst_fd < 0 {
        libc::close(src_fd);
        return -1;
    }

    let buffer_size = if buffer_size == 0 { 65536 } else { buffer_size };
    let mut buf = Vec::with_capacity(buffer_size);
    buf.resize(buffer_size, 0u8);
    let mut total: i64 = 0;

    loop {
        let n = libc::read(src_fd, buf.as_mut_ptr() as *mut libc::c_void, buffer_size);
        if n < 0 {
            libc::close(src_fd);
            libc::close(dst_fd);
            return -1;
        }
        if n == 0 {
            break;
        }
        let mut written = 0i64;
        while written < n as i64 {
            let w = libc::write(dst_fd, buf.as_mut_ptr().offset(written as isize) as *mut libc::c_void, (n as i64 - written) as size_t);
            if w < 0 {
                libc::close(src_fd);
                libc::close(dst_fd);
                return -1;
            }
            written += w as i64;
        }
        total += n as i64;
    }

    libc::close(src_fd);
    libc::close(dst_fd);
    total
}

/// Get file size
#[no_mangle]
pub unsafe extern "C" fn rust_file_size(path: *const c_char) -> i64 {
    let mut stat: libc::stat = std::mem::zeroed();
    if libc::stat(path, &mut stat) < 0 {
        return -1;
    }
    stat.st_size
}

/// Check if file exists
#[no_mangle]
pub unsafe extern "C" fn rust_file_exists(path: *const c_char) -> c_int {
    let mut stat: libc::stat = std::mem::zeroed();
    if libc::stat(path, &mut stat) < 0 {
        0
    } else {
        1
    }
}

/// Create directory recursively
#[no_mangle]
pub unsafe extern "C" fn rust_create_dirs(path: *const c_char) -> c_int {
    let path_str = std::ffi::CStr::from_ptr(path);
    let path_slice = match path_str.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let mut current = std::path::PathBuf::new();
    for comp in std::path::Path::new(path_slice).components() {
        current.push(comp);
        let c_str = std::ffi::CString::new(current.to_string_lossy().as_ref()).unwrap();
        if libc::mkdir(c_str.as_ptr(), 0o755) < 0 {
            // Check if it already exists - if so, continue
            // We can't easily get errno on Android, so just continue
        }
    }

    0
}

/// Delete file
#[no_mangle]
pub unsafe extern "C" fn rust_delete_file(path: *const c_char) -> c_int {
    libc::unlink(path)
}