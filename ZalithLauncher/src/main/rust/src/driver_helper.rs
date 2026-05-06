//
// Driver helper module for Adreno GPU detection and Turnip Vulkan loading.
// Based on nsbypass.c and driver_helper.c
//
// Created by Vera-Firefly on 17.01.2025.
//

use libc::c_char;
use std::ptr;

#[cfg(target_arch = "aarch64")]
use libc::mprotect;

#[cfg(target_arch = "aarch64")]
const PAGE_SIZE: usize = 4096;

// Constants from the original C code
#[allow(dead_code)]
const SEARCH_PATH: &str = "/system/lib64";

// ELF structures for ARM64
#[cfg(target_arch = "aarch64")]
#[repr(C)]
struct Elf64_Ehdr {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

#[cfg(target_arch = "aarch64")]
#[repr(C)]
struct Elf64_Shdr {
    sh_name: u32,
    sh_type: u32,
    sh_flags: u64,
    sh_addr: u64,
    sh_offset: u64,
    sh_size: u64,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u64,
    sh_entsize: u64,
}

#[cfg(target_arch = "aarch64")]
#[repr(C)]
struct Elf64_Dyn {
    d_tag: i64,
    d_val: u64,
}

const SHT_DYNAMIC: u32 = 6;
const DT_SONAME: i64 = 14;

// Bit manipulation for ARM branch instructions
const OP_MS: u32 = 0b11111100000000000000000000000000;
const BL_OP: u32 = 0b10010100000000000000000000000000;
const BL_IM: u32 = 0b00000011111111111111111111111111;

#[cfg(target_arch = "aarch64")]
unsafe fn find_branch_label(func_start: *mut libc::c_void) -> *mut libc::c_void {
    let func_page_start = (func_start as usize & !(PAGE_SIZE - 1)) as *mut libc::c_void;
    mprotect(func_page_start, PAGE_SIZE, libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC);

    let mut bl_addr = func_start as *mut u32;

    while (*bl_addr & OP_MS) != BL_OP {
        bl_addr = bl_addr.add(1);
    }

    let offset = (*bl_addr & BL_IM) as isize * 4;
    (bl_addr as *mut c_char).offset(offset) as *mut libc::c_void
}

pub fn linker_ns_load(lib_search_path: *const c_char) -> bool {
    // This feature is only available on ARM64 with ADRENO_POSSIBLE
    // For now, we return false as the Rust implementation would need more
    // complete android namespace API access that's not available via libc
    let _ = lib_search_path;
    false
}

pub unsafe fn linker_ns_dlopen(name: *const c_char, _flag: libc::c_int) -> *mut libc::c_void {
    if !cfg!(target_arch = "aarch64") {
        return ptr::null_mut();
    }
    let _ = name;
    // Placeholder - full implementation would need android_dlext.h bindings
    ptr::null_mut()
}

pub unsafe fn linker_ns_dlopen_unique(
    _tmpdir: *const c_char,
    _name: *const c_char,
    _flags: libc::c_int,
) -> *mut libc::c_void {
    if !cfg!(target_arch = "aarch64") {
        return ptr::null_mut();
    }
    // Placeholder - full implementation would need elf patching
    ptr::null_mut()
}

#[cfg(target_arch = "aarch64")]
unsafe fn patch_elf_soname_impl(
    patchfd: libc::c_int,
    realfd: libc::c_int,
    patchid: u16,
) -> bool {
    use libc::{close, fstat, ftruncate64, malloc, memcpy, mmap, munmap, read, snprintf};

    let mut stat_buf: libc::stat = std::mem::zeroed();
    if fstat(realfd, &mut stat_buf) != 0 {
        return false;
    }

    let file_size = stat_buf.st_size as usize;
    if ftruncate64(patchfd, file_size as i64) == -1 {
        return false;
    }

    let target = mmap(
        ptr::null_mut(),
        file_size,
        libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_SHARED,
        patchfd,
        0,
    );
    if target.is_null() {
        return false;
    }

    let bytes_read = read(realfd, target, file_size);
    if bytes_read != file_size as isize {
        munmap(target, file_size);
        return false;
    }
    close(realfd);

    let ehdr = target as *mut Elf64_Ehdr;
    let shdr = (target as *mut u8).add((*ehdr).e_shoff as usize) as *mut Elf64_Shdr;

    for i in 0..(*ehdr).e_shnum {
        let hdr = &*shdr.add(i as usize);
        if hdr.sh_type == SHT_DYNAMIC {
            let link_idx = hdr.sh_link as usize;
            let strtab = (target as *mut u8).add((&*shdr.add(link_idx)).sh_offset as usize);
            let dyn_entries = (target as *mut u8).add(hdr.sh_offset as usize) as *mut Elf64_Dyn;
            let num_entries = hdr.sh_size as usize / hdr.sh_entsize as usize;

            for k in 0..num_entries {
                let dyn_entry = &*dyn_entries.add(k);
                if dyn_entry.d_tag == DT_SONAME {
                    let soname = strtab.add(dyn_entry.d_val as usize);
                    let sprb_ptr = malloc(4) as *mut c_char;
                    if !sprb_ptr.is_null() {
                        snprintf(sprb_ptr, 4, b"%03x\0".as_ptr() as *const c_char, patchid as libc::c_int);
                        memcpy(soname as *mut libc::c_void, sprb_ptr as *const libc::c_void, 3);
                        libc::free(sprb_ptr as *mut libc::c_void);
                    }
                    munmap(target, file_size);
                    return true;
                }
            }
        }
    }

    munmap(target, file_size);
    false
}

#[cfg(not(target_arch = "aarch64"))]
unsafe fn patch_elf_soname_impl(
    _patchfd: libc::c_int,
    _realfd: libc::c_int,
    _patchid: u16,
) -> bool {
    false
}

// Public API function for C compatibility
#[no_mangle]
pub unsafe extern "C" fn linker_ns_load_c(lib_search_path: *const c_char) -> libc::c_int {
    if linker_ns_load(lib_search_path) {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn linker_ns_dlopen_c(name: *const c_char, flag: libc::c_int) -> *mut libc::c_void {
    linker_ns_dlopen(name, flag)
}

#[no_mangle]
pub unsafe extern "C" fn linker_ns_dlopen_unique_c(
    tmpdir: *const c_char,
    name: *const c_char,
    flags: libc::c_int,
) -> *mut libc::c_void {
    linker_ns_dlopen_unique(tmpdir, name, flags)
}