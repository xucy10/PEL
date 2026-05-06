//! CPU big core affinity - Rust implementation
//!
//! Finds and sets CPU affinity to the fastest core using safe Rust.

use libc::{c_char, c_ulong, close, open, read, sched_setaffinity, cpu_set_t, strerror, O_RDONLY};
use core::ffi::CStr;
use core::fmt;

// CPU_SETSIZE is typically 1024 on Linux
const CPU_SETSIZE: usize = 1024;
const FREQ_MAX: usize = 256;
const PATH_MAX: usize = 4096;

#[derive(Debug, PartialEq)]
pub enum BigCoreError {
    OpenFailed,
    ReadFailed,
    ParseFailed,
    NoCoresFound,
    SchedAffinityFailed(i32),
}

impl fmt::Display for BigCoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BigCoreError::OpenFailed => write!(f, "Failed to open CPU freq file"),
            BigCoreError::ReadFailed => write!(f, "Failed to read CPU frequency"),
            BigCoreError::ParseFailed => write!(f, "Failed to parse frequency"),
            BigCoreError::NoCoresFound => write!(f, "No CPU cores found"),
            BigCoreError::SchedAffinityFailed(err) => write!(f, "Failed to set CPU affinity: {}", err),
        }
    }
}

/// Format the CPU frequency path for a given core
fn format_cpu_path(cpu_core: u32) -> [u8; PATH_MAX] {
    let mut buffer = [0u8; PATH_MAX];
    let path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/cpuinfo_max_freq\0", cpu_core);
    let path_bytes = path.as_bytes();
    let len = path_bytes.len().min(PATH_MAX);
    buffer[..len].copy_from_slice(&path_bytes[..len]);
    buffer
}

/// Read CPU frequency for a given core
fn read_cpu_freq(core_id: u32) -> Result<c_ulong, BigCoreError> {
    let path_buffer = format_cpu_path(core_id);
    let path_ptr = path_buffer.as_ptr() as *const c_char;

    // Safety: path_buffer is a valid null-terminated C string
    let fd = unsafe { open(path_ptr, O_RDONLY) };
    if fd == -1 {
        return Err(BigCoreError::OpenFailed);
    }

    let mut freq_buffer = [0u8; FREQ_MAX];
    let read_count = unsafe { read(fd, freq_buffer.as_mut_ptr() as *mut libc::c_void, FREQ_MAX) };

    // Close the file descriptor
    unsafe { close(fd) };

    if read_count <= 0 {
        return Err(BigCoreError::ReadFailed);
    }

    // Null-terminate the buffer
    freq_buffer[read_count as usize] = 0;

    // Parse frequency from string using strtoul equivalent
    let freq_str = unsafe {
        CStr::from_ptr(freq_buffer.as_ptr() as *const c_char)
            .to_str()
            .map_err(|_| BigCoreError::ParseFailed)?
    };

    let core_freq = freq_str.trim()
        .parse::<c_ulong>()
        .map_err(|_| BigCoreError::ParseFailed)?;

    Ok(core_freq)
}

/// Find the big core (core with highest frequency)
fn find_big_core() -> Result<(u32, c_ulong), BigCoreError> {
    let mut max_freq: c_ulong = 0;
    let mut big_core_id: u32 = 0;
    let mut core_count: u32 = 0;

    loop {
        match read_cpu_freq(core_count) {
            Ok(core_freq) => {
                if core_freq >= max_freq {
                    max_freq = core_freq;
                    big_core_id = core_count;
                }
                core_count += 1;
            }
            Err(BigCoreError::OpenFailed) => {
                // Stop when we can't open the next core
                break;
            }
            Err(e) => return Err(e),
        }
    }

    if core_count == 0 {
        Err(BigCoreError::NoCoresFound)
    } else {
        Ok((big_core_id, max_freq))
    }
}

/// Set the current thread's affinity to the big core
#[allow(dead_code)]
pub fn set_affinity() -> Result<(), BigCoreError> {
    let (big_core_id, max_freq) = find_big_core()?;

    // Print information (matches C code output)
    println!("bigcore: big CPU number is {}, frequency {} Hz", big_core_id, max_freq);

    // Create CPU set and set affinity
    let mut bigcore_affinity_set: cpu_set_t = unsafe { core::mem::zeroed() };

    // Safety: CPU_SET is a macro in C, we need to call the _S variant
    unsafe {
        libc::CPU_SET(big_core_id as usize, &mut bigcore_affinity_set);
    }

    let result = unsafe { sched_setaffinity(0, core::mem::size_of::<cpu_set_t>(), &bigcore_affinity_set) };

    if result != 0 {
        let error_str = unsafe {
            let err_ptr = strerror(result);
            if !err_ptr.is_null() {
                CStr::from_ptr(err_ptr).to_string_lossy().into_owned()
            } else {
                format!("Unknown error {}", result)
            }
        };
        println!("bigcore: setting affinity failed: {}", error_str);
        Err(BigCoreError::SchedAffinityFailed(result))
    } else {
        println!("bigcore: forced current thread onto big core");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_big_core() {
        let result = find_big_core();
        assert!(result.is_ok() || result == Err(BigCoreError::NoCoresFound));
    }
}