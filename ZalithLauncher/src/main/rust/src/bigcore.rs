//! Big Core Affinity - CPU core placement for Android
//!
//! Sets CPU affinity to prefer big cores for better performance.
//! Uses safe wrappers around libc functions.

use libc::{c_int, sched_setaffinity, CPU_SET, CPU_ZERO};

/// Set CPU affinity to prefer big (performance) cores
/// Returns Ok(()) on success, Err(errno) on failure
pub fn set_affinity() -> Result<(), c_int> {
    unsafe {
        let mut cpuset: libc::cpu_set_t = std::mem::zeroed();
        CPU_ZERO(&mut cpuset);

        // Try to set affinity to big cores (typically cores 4-7 on big.LITTLE)
        // This is a best-effort approach - actual core mapping varies by device
        for core in [4, 5, 6, 7, 0, 1, 2, 3] {
            CPU_SET(core as usize, &mut cpuset);
        }

        let tid = libc::pthread_self();
        if sched_setaffinity(tid as c_int, std::mem::size_of::<libc::cpu_set_t>(), &cpuset) != 0 {
            return Err(*libc::__errno_location());
        }

        Ok(())
    }
}