//! SHA1 hash implementation in pure Rust for high performance
//!
//! This module provides faster SHA1 computation compared to Java's
//! MessageDigest by using optimized Rust implementation.

use libc::c_char;

/// Buffer size for reading files
const READ_BUFFER_SIZE: usize = 65536;

/// SHA1 digest output (20 bytes)
pub type Sha1Digest = [u8; 20];

#[inline(always)]
fn rol32(value: u32, bits: u32) -> u32 {
    (value << bits) | (value >> (32 - bits))
}

fn sha1_block(state: &mut [u32; 5], block: &[u8; 64]) {
    let mut w = [0u32; 16];

    // Prepare message schedule
    for i in 0..16 {
        w[i] = u32::from_be_bytes([
            block[i * 4],
            block[i * 4 + 1],
            block[i * 4 + 2],
            block[i * 4 + 3],
        ]);
    }
    for i in 16..80 {
        w[i] = rol32(w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16], 1);
    }

    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];

    // 80 rounds
    for i in 0..20 {
        let temp = rol32(a, 5)
            .wrapping_add((b & c) | ((!b) & d))
            .wrapping_add(e)
            .wrapping_add(w[i])
            .wrapping_add(0x5A827999);
        e = d;
        d = c;
        c = rol32(b, 30);
        b = a;
        a = temp;
    }
    for i in 20..40 {
        let temp = rol32(a, 5)
            .wrapping_add(b ^ c ^ d)
            .wrapping_add(e)
            .wrapping_add(w[i])
            .wrapping_add(0x6ED9EBA1);
        e = d;
        d = c;
        c = rol32(b, 30);
        b = a;
        a = temp;
    }
    for i in 40..60 {
        let temp = rol32(a, 5)
            .wrapping_add((b & c) | (b & d) | (c & d))
            .wrapping_add(e)
            .wrapping_add(w[i])
            .wrapping_add(0x8F1BBCDC);
        e = d;
        d = c;
        c = rol32(b, 30);
        b = a;
        a = temp;
    }
    for i in 60..80 {
        let temp = rol32(a, 5)
            .wrapping_add(b ^ c ^ d)
            .wrapping_add(e)
            .wrapping_add(w[i])
            .wrapping_add(0xCA62C1D6);
        e = d;
        d = c;
        c = rol32(b, 30);
        b = a;
        a = temp;
    }

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
}

/// Compute SHA1 hash of a byte array
pub fn sha1(data: &[u8]) -> Sha1Digest {
    let mut state = [0x67452301u32, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
    let mut count: u64 = 0;
    let len = data.len();
    let mut offset = 0;

    // Process complete 64-byte blocks
    while offset + 64 <= len {
        let block: [u8; 64] = data[offset..offset + 64].try_into().unwrap();
        sha1_block(&mut state, &block);
        count += 64;
        offset += 64;
    }

    // Handle remaining bytes with padding
    let remaining = len - offset;
    if remaining > 0 {
        count += remaining as u64;
    }

    // Padding
    let bit_count = count * 8;
    let final_offset = (count as usize) % 64;
    let mut padded = [0u8; 64];

    if remaining > 0 {
        padded[..remaining].copy_from_slice(&data[offset..]);
    }

    if final_offset < 56 {
        padded[final_offset] = 0x80;
        padded[56] = ((bit_count >> 56) & 0xFF) as u8;
        padded[57] = ((bit_count >> 48) & 0xFF) as u8;
        padded[58] = ((bit_count >> 40) & 0xFF) as u8;
        padded[59] = ((bit_count >> 32) & 0xFF) as u8;
        padded[60] = ((bit_count >> 24) & 0xFF) as u8;
        padded[61] = ((bit_count >> 16) & 0xFF) as u8;
        padded[62] = ((bit_count >> 8) & 0xFF) as u8;
        padded[63] = (bit_count & 0xFF) as u8;
    } else {
        // Need two blocks
        padded[final_offset] = 0x80;
        sha1_block(&mut state, &padded);
        padded.fill(0);
        padded[56] = ((bit_count >> 56) & 0xFF) as u8;
        padded[57] = ((bit_count >> 48) & 0xFF) as u8;
        padded[58] = ((bit_count >> 40) & 0xFF) as u8;
        padded[59] = ((bit_count >> 32) & 0xFF) as u8;
        padded[60] = ((bit_count >> 24) & 0xFF) as u8;
        padded[61] = ((bit_count >> 16) & 0xFF) as u8;
        padded[62] = ((bit_count >> 8) & 0xFF) as u8;
        padded[63] = (bit_count & 0xFF) as u8;
    }

    sha1_block(&mut state, &padded);

    // Output as 20 bytes
    let mut result = [0u8; 20];
    for i in 0..5 {
        result[i * 4] = (state[i] >> 24) as u8;
        result[i * 4 + 1] = (state[i] >> 16) as u8;
        result[i * 4 + 2] = (state[i] >> 8) as u8;
        result[i * 4 + 3] = state[i] as u8;
    }
    result
}

/// Compute SHA1 hash of a byte array and return hex string
pub fn sha1_hex(data: &[u8]) -> String {
    sha1(data)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

/// Compute SHA1 hash of a file and return hex string
pub fn sha1_file_hex(path: *const c_char) -> Option<String> {
    use std::io::Read;

    unsafe {
        let path_str = std::ffi::CStr::from_ptr(path as *const c_char);
        let file_path = path_str.to_str().ok()?;

        let file = std::fs::File::open(file_path).ok()?;
        let mut reader = std::io::BufReader::with_capacity(READ_BUFFER_SIZE, file);
        let mut buffer = [0u8; READ_BUFFER_SIZE];

        let mut state = [0x67452301u32, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
        let mut count: u64 = 0;
        let mut pending = [0u8; 64];
        let mut pending_len = 0usize;

        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    let mut offset = 0;
                    while offset < n {
                        if pending_len == 64 {
                            sha1_block(&mut state, &pending);
                            count += 64;
                            pending_len = 0;
                        }
                        let to_copy = 64 - pending_len;
                        let remaining = n - offset;
                        let copy = if to_copy > remaining { remaining } else { to_copy };
                        pending[pending_len..][..copy].copy_from_slice(&buffer[offset..][..copy]);
                        pending_len += copy;
                        offset += copy;
                    }
                }
                Err(_) => return None,
            }
        }

        // Padding
        let bit_count = count * 8;
        pending[pending_len] = 0x80;
        pending[pending_len + 1..].fill(0);

        if pending_len < 56 {
            pending[56] = ((bit_count >> 56) & 0xFF) as u8;
            pending[57] = ((bit_count >> 48) & 0xFF) as u8;
            pending[58] = ((bit_count >> 40) & 0xFF) as u8;
            pending[59] = ((bit_count >> 32) & 0xFF) as u8;
            pending[60] = ((bit_count >> 24) & 0xFF) as u8;
            pending[61] = ((bit_count >> 16) & 0xFF) as u8;
            pending[62] = ((bit_count >> 8) & 0xFF) as u8;
            pending[63] = (bit_count & 0xFF) as u8;
            sha1_block(&mut state, &pending);
        } else {
            sha1_block(&mut state, &pending);
            pending.fill(0);
            pending[56] = ((bit_count >> 56) & 0xFF) as u8;
            pending[57] = ((bit_count >> 48) & 0xFF) as u8;
            pending[58] = ((bit_count >> 40) & 0xFF) as u8;
            pending[59] = ((bit_count >> 32) & 0xFF) as u8;
            pending[60] = ((bit_count >> 24) & 0xFF) as u8;
            pending[61] = ((bit_count >> 16) & 0xFF) as u8;
            pending[62] = ((bit_count >> 8) & 0xFF) as u8;
            pending[63] = (bit_count & 0xFF) as u8;
            sha1_block(&mut state, &pending);
        }

        Some(state.iter().map(|v| format!("{:08x}", v)).collect())
    }
}

/// Verify file SHA1 against expected hex string
pub fn sha1_verify_file(path: *const c_char, expected_hex: &str) -> bool {
    sha1_file_hex(path)
        .map(|computed| computed.eq_ignore_ascii_case(expected_hex))
        .unwrap_or(false)
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha1_empty_string() {
        let hash = sha1_hex(b"");
        assert_eq!(hash, "da39a3ee5e6b4b0d3255bfef95601890afd80709");
    }

    #[test]
    fn test_sha1_hello_world() {
        let hash = sha1_hex(b"hello world");
        assert_eq!(hash, "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");
    }

    #[test]
    fn test_sha1_abc() {
        let hash = sha1_hex(b"abc");
        assert_eq!(hash, "a9993e364706816aba3e25717850c26c9cd0d89d");
    }

    #[test]
    fn test_sha1_abc_bytes() {
        let digest = sha1(b"abc");
        assert_eq!(digest.len(), 20);
    }
}