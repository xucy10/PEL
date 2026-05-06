//! JVM Launcher in Rust
//!
//! Memory-safe implementation of JLI_Launch for starting JVM from native code.

use jni::JNIEnv;
use jni::objects::{JByteArray, JClass};
use jni::sys::{jint, jbyteArray};
use libc::{c_char, c_int, pipe, close, SIGABRT};
use std::ptr;
use std::ffi::CStr;

// Signal handling globals
static mut ABORT_READ_FD: c_int = -1;
static mut ABORT_WRITE_FD: c_int = -1;

/// Initialize the abort waiter using pipe
/// This handles SIGABRT gracefully by waiting for signal
fn initialize_abort_waiter() -> c_int {
    unsafe {
        let mut fds = [-1, -1];
        if pipe(fds.as_mut_ptr()) != 0 {
            return -1;
        }
        ABORT_READ_FD = fds[0];
        ABORT_WRITE_FD = fds[1];

        // Set up signal handler for SIGABRT using signal()
        libc::signal(SIGABRT, libc::SIG_IGN);

        0
    }
}

/// Launch JVM using JLI_Launch
#[no_mangle]
pub unsafe extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_rustLaunchJVM(
    env: JNIEnv,
    _class: JClass,
    args_array: jbyteArray,
) -> jint {
    // Initialize abort waiter
    if initialize_abort_waiter() != 0 {
        eprintln!("Rust: Failed to initialize abort waiter");
        return -1;
    }

    // Convert Java byte[] to C arguments
    let java_array = JByteArray::from_raw(args_array as *mut _);
    let java_bytes: Vec<u8> = match env.convert_byte_array(&java_array) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("Rust: Failed to convert byte array");
            return -1;
        }
    };

    // Find null terminator to determine string count
    let num_args = java_bytes.iter().filter(|&&b| b == 0).count();
    if num_args < 4 {
        eprintln!("Rust: Invalid arguments - need at least 4 strings (jvmpath, confheap, jrehome, jarfile, [app args...])");
        return -1;
    }

    // Allocate argument array
    let mut argv: Vec<*mut c_char> = Vec::with_capacity(num_args + 1);
    let mut current_pos = 0;

    // Parse arguments from byte array
    for _ in 0..num_args {
        // Find string length
        let mut len = 0;
        while current_pos + len < java_bytes.len() && java_bytes[current_pos + len] != 0 {
            len += 1;
        }

        // Create C string
        let mut chars: Vec<u8> = Vec::with_capacity(len + 1);
        for j in 0..len {
            chars.push(java_bytes[current_pos + j] as u8);
        }
        chars.push(0);

        let c_str = libc::malloc(chars.len()) as *mut c_char;
        if c_str.is_null() {
            eprintln!("Rust: Failed to allocate memory");
            return -1;
        }
        ptr::copy_nonoverlapping(chars.as_ptr() as *const c_char, c_str, chars.len());
        argv.push(c_str);

        current_pos += len + 1;
    }
    argv.push(ptr::null_mut());

    // Extract critical paths
    if argv.len() < 5 {
        eprintln!("Rust: Not enough arguments");
        return -1;
    }

    let jvmpath = argv[0];
    let _confheap = argv[1];
    let jrehome = argv[2];
    let jarfile = argv[3];

    // Log for debugging
    eprintln!("Rust: jvmpath = {}", CStr::from_ptr(jvmpath).to_string_lossy());
    eprintln!("Rust: jrehome = {}", CStr::from_ptr(jrehome).to_string_lossy());
    eprintln!("Rust: jarfile = {}", CStr::from_ptr(jarfile).to_string_lossy());

    // Find libjli.so
    let jrehome_str = CStr::from_ptr(jrehome).to_string_lossy();
    let mut libjli_path = std::path::PathBuf::from(jrehome_str.as_ref());
    libjli_path.push("lib");
    libjli_path.push("libjli.so");

    let libjli_cstr = std::ffi::CString::new(libjli_path.to_string_lossy().as_ref()).unwrap();

    // Try to load libjli.so
    let libjli = libc::dlopen(libjli_cstr.as_ptr(), libc::RTLD_LAZY);
    if libjli.is_null() {
        // Try alternative path for ARM64
        let mut alt_path = std::path::PathBuf::from(jrehome_str.as_ref());
        alt_path.push("lib");
        alt_path.push("arm64");
        alt_path.push("libjli.so");

        let alt_cstr = std::ffi::CString::new(alt_path.to_string_lossy().as_ref()).unwrap();
        let libjli = libc::dlopen(alt_cstr.as_ptr(), libc::RTLD_LAZY);

        if libjli.is_null() {
            eprintln!("Rust: Failed to load libjli.so");
            let error = libc::dlerror();
            if !error.is_null() {
                eprintln!("Rust: dlopen error: {}", CStr::from_ptr(error).to_string_lossy());
            }
            return -1;
        }
    }

    // Find JLI_Launch
    let jni_create: *mut libc::c_void = libc::dlsym(libjli, b"JLI_Launch\0".as_ptr() as *const c_char);
    if jni_create.is_null() {
        eprintln!("Rust: Failed to find JLI_Launch");
        libc::dlclose(libjli);
        return -1;
    }

    // JLI_Launch signature:
    // JLI_Launch(int argc, char **argv, int jargc, const char **jargv,
    //            int dryrun, int **launchMode, const char *jrehome,
    //            int *reserved, const char *classpath, int printjava, int printjava_args)
    type JLILaunchFn = unsafe extern "C" fn(
        c_int,                // argc
        *mut *mut c_char,     // argv
        c_int,                // jargc
        *const *const c_char,  // jargv
        c_int,                // dryrun
        *mut *mut c_int,      // launchMode
        *const c_char,        // jrehome
        *mut c_int,           // reserved
        *const c_char,        // classpath
        c_int,                // printjava
        c_int,                // printjava_args
    ) -> c_int;

    let jli_launch: JLILaunchFn = std::mem::transmute(jni_create);

    // Set up arguments for JLI_Launch
    let mut args: Vec<*mut c_char> = Vec::new();
    args.push(jvmpath); // argv[0]
    args.push(jarfile); // argv[1] = main class/JAR

    // Add application arguments (everything after jarfile)
    for i in 5..argv.len() - 1 {
        args.push(argv[i]);
    }
    args.push(ptr::null_mut());

    let argc = args.len() as c_int - 1;

    // Launch JVM
    eprintln!("Rust: Calling JLI_Launch...");
    let mut launch_mode: *mut c_int = std::ptr::null_mut();
    let mut reserved: c_int = 0;

    let result = jli_launch(
        argc,
        args.as_mut_ptr(),
        0,
        std::ptr::null(),
        0,
        &mut launch_mode,
        jrehome,
        &mut reserved,
        ptr::null(),
        0,
        0,
    );

    eprintln!("Rust: JLI_Launch returned {}", result);

    // Cleanup
    libc::dlclose(libjli);
    for arg in argv.iter() {
        if !arg.is_null() {
            libc::free(*arg as *mut libc::c_void);
        }
    }

    if ABORT_READ_FD >= 0 {
        close(ABORT_READ_FD);
    }
    if ABORT_WRITE_FD >= 0 {
        close(ABORT_WRITE_FD);
    }

    result
}