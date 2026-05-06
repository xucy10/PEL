//! SHA1 implementation in pure Rust
//!
//! Optimized for performance with minimal memory allocation.
//! Uses inline functions and loop unrolling for maximum throughput.

/// Rotate left operation
#[inline(always)]
fn rol32(value: u32, bits: u32) -> u32 {
    (value << bits) | (value >> (32 - bits))
}

// SHA1 round constants
const K0: u32 = 0x5A827999;
const K1: u32 = 0x6ED9EBA1;
const K2: u32 = 0x8F1BBCDC;
const K3: u32 = 0xCA62C1D6;

/// Single SHA1 round operation - fully inlined for optimization
#[inline(always)]
fn sha1_round(a: u32, b: u32, c: u32, d: u32, e: u32, w: u32, k: u32) -> (u32, u32, u32, u32, u32) {
    let temp = rol32(a, 5)
        .wrapping_add(e)
        .wrapping_add(w)
        .wrapping_add(k);
    (
        temp.wrapping_add((b & c) | ((!b) & d)),
        a,
        rol32(b, 30),
        c,
        d
    )
}

/// Process a single 64-byte block
#[inline(always)]
fn sha1_block(state: &mut [u32; 5], block: &[u8; 64]) {
    let mut w = [0u32; 80];

    // Message schedule preparation - first 16 words from block
    // Unrolled for better instruction-level parallelism
    w[0] = u32::from_be_bytes([block[0], block[1], block[2], block[3]]);
    w[1] = u32::from_be_bytes([block[4], block[5], block[6], block[7]]);
    w[2] = u32::from_be_bytes([block[8], block[9], block[10], block[11]]);
    w[3] = u32::from_be_bytes([block[12], block[13], block[14], block[15]]);
    w[4] = u32::from_be_bytes([block[16], block[17], block[18], block[19]]);
    w[5] = u32::from_be_bytes([block[20], block[21], block[22], block[23]]);
    w[6] = u32::from_be_bytes([block[24], block[25], block[26], block[27]]);
    w[7] = u32::from_be_bytes([block[28], block[29], block[30], block[31]]);
    w[8] = u32::from_be_bytes([block[32], block[33], block[34], block[35]]);
    w[9] = u32::from_be_bytes([block[36], block[37], block[38], block[39]]);
    w[10] = u32::from_be_bytes([block[40], block[41], block[42], block[43]]);
    w[11] = u32::from_be_bytes([block[44], block[45], block[46], block[47]]);
    w[12] = u32::from_be_bytes([block[48], block[49], block[50], block[51]]);
    w[13] = u32::from_be_bytes([block[52], block[53], block[54], block[55]]);
    w[14] = u32::from_be_bytes([block[56], block[57], block[58], block[59]]);
    w[15] = u32::from_be_bytes([block[60], block[61], block[62], block[63]]);

    // Remaining words using recurrence
    for i in 16..80 {
        w[i] = rol32(w[i-3] ^ w[i-8] ^ w[i-14] ^ w[i-16], 1);
    }

    // Initialize working variables
    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];

    // 80 rounds with loop unrolling
    // Rounds 0-19 (K0)
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[0], K0);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[1], K0);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[2], K0);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[3], K0);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[4], K0);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[5], K0);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[6], K0);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[7], K0);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[8], K0);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[9], K0);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[10], K0);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[11], K0);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[12], K0);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[13], K0);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[14], K0);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[15], K0);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[16], K0);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[17], K0);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[18], K0);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[19], K0);

    // Rounds 20-39 (K1)
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[20], K1);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[21], K1);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[22], K1);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[23], K1);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[24], K1);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[25], K1);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[26], K1);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[27], K1);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[28], K1);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[29], K1);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[30], K1);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[31], K1);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[32], K1);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[33], K1);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[34], K1);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[35], K1);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[36], K1);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[37], K1);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[38], K1);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[39], K1);

    // Rounds 40-59 (K2)
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[40], K2);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[41], K2);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[42], K2);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[43], K2);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[44], K2);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[45], K2);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[46], K2);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[47], K2);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[48], K2);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[49], K2);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[50], K2);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[51], K2);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[52], K2);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[53], K2);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[54], K2);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[55], K2);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[56], K2);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[57], K2);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[58], K2);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[59], K2);

    // Rounds 60-79 (K3)
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[60], K3);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[61], K3);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[62], K3);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[63], K3);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[64], K3);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[65], K3);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[66], K3);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[67], K3);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[68], K3);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[69], K3);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[70], K3);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[71], K3);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[72], K3);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[73], K3);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[74], K3);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[75], K3);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[76], K3);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[77], K3);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[78], K3);
    (a, b, c, d, e) = sha1_round(a, b, c, d, e, w[79], K3);

    // Update state
    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
}

/// Pad data to 64-byte boundary and create final block with padding
#[inline(always)]
fn pad_data(data: &[u8]) -> ([u8; 64], usize) {
    let len = data.len();
    let mut padded = [0u8; 64];

    // Copy available bytes
    let copy_len = if len < 56 { len } else { 56 };
    padded[..copy_len].copy_from_slice(&data[..copy_len]);

    // Add 0x80 after data
    if len < 56 {
        padded[len] = 0x80;
    }

    (padded, len)
}

/// Convert state to 20-byte hash
#[inline(always)]
fn state_to_hash(state: &[u32; 5]) -> [u8; 20] {
    let mut result = [0u8; 20];
    for (i, &v) in state.iter().enumerate() {
        result[i*4] = (v >> 24) as u8;
        result[i*4+1] = (v >> 16) as u8;
        result[i*4+2] = (v >> 8) as u8;
        result[i*4+3] = v as u8;
    }
    result
}

/// Compute SHA1 hash of data
pub fn sha1(data: &[u8]) -> [u8; 20] {
    let mut state = [0x67452301u32, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
    let len = data.len();

    // Process full 64-byte blocks
    let full_blocks = len / 64;
    for i in 0..full_blocks {
        let block: [u8; 64] = data[i*64..i*64+64].try_into().unwrap();
        sha1_block(&mut state, &block);
    }

    // Handle remaining bytes
    let remaining = len % 64;
    if remaining > 0 {
        let (mut padded, _) = pad_data(&data[full_blocks*64..]);
        let offset = full_blocks * 64;
        padded[..remaining].copy_from_slice(&data[offset..offset+remaining]);

        // Set bit length
        let bit_len = (len as u64).wrapping_mul(8);
        padded[56] = (bit_len >> 56) as u8;
        padded[57] = (bit_len >> 48) as u8;
        padded[58] = (bit_len >> 40) as u8;
        padded[59] = (bit_len >> 32) as u8;
        padded[60] = (bit_len >> 24) as u8;
        padded[61] = (bit_len >> 16) as u8;
        padded[62] = (bit_len >> 8) as u8;
        padded[63] = bit_len as u8;

        sha1_block(&mut state, &padded);
    } else if len > 0 {
        // Exact multiple of 64 - add padding block
        let mut padded = [0u8; 64];
        padded[0] = 0x80;
        let bit_len = (len as u64).wrapping_mul(8);
        padded[56] = (bit_len >> 56) as u8;
        padded[57] = (bit_len >> 48) as u8;
        padded[58] = (bit_len >> 40) as u8;
        padded[59] = (bit_len >> 32) as u8;
        padded[60] = (bit_len >> 24) as u8;
        padded[61] = (bit_len >> 16) as u8;
        padded[62] = (bit_len >> 8) as u8;
        padded[63] = bit_len as u8;
        sha1_block(&mut state, &padded);
    }

    state_to_hash(&state)
}

/// Compute SHA1 and return as hex string
pub fn sha1_hex(data: &[u8]) -> String {
    let hash = sha1(data);
    format!(
        "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        hash[0], hash[1], hash[2], hash[3], hash[4],
        hash[5], hash[6], hash[7], hash[8], hash[9],
        hash[10], hash[11], hash[12], hash[13], hash[14],
        hash[15], hash[16], hash[17], hash[18], hash[19]
    )
}

/// Compute SHA1 hash of a file
pub fn sha1_file_hex(path: *const libc::c_char) -> Option<String> {
    use std::io::Read;

    unsafe {
        let path_str = std::ffi::CStr::from_ptr(path);
        let file_path = path_str.to_str().ok()?;
        let file = std::fs::File::open(file_path).ok()?;
        let mut reader = std::io::BufReader::with_capacity(65536, file);
        let mut state = [0x67452301u32, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
        let mut buf = [0u8; 65536];
        let mut total_len = 0u64;

        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    total_len += n as u64;
                    let mut offset = 0;
                    while offset + 64 <= n {
                        let block: [u8; 64] = buf[offset..offset+64].try_into().unwrap();
                        sha1_block(&mut state, &block);
                        offset += 64;
                    }
                    // Handle remaining bytes
                    if offset < n {
                        let remaining = n - offset;
                        let mut padded = [0u8; 64];
                        padded[..remaining].copy_from_slice(&buf[offset..n]);
                        if remaining < 56 {
                            padded[remaining] = 0x80;
                        }
                        let bit_len = total_len * 8;
                        padded[56] = (bit_len >> 56) as u8;
                        padded[57] = (bit_len >> 48) as u8;
                        padded[58] = (bit_len >> 40) as u8;
                        padded[59] = (bit_len >> 32) as u8;
                        padded[60] = (bit_len >> 24) as u8;
                        padded[61] = (bit_len >> 16) as u8;
                        padded[62] = (bit_len >> 8) as u8;
                        padded[63] = bit_len as u8;
                        sha1_block(&mut state, &padded);
                    }
                }
                Err(_) => return None,
            }
        }

        Some(format!(
            "{:08x}{:08x}{:08x}{:08x}{:08x}",
            state[0], state[1], state[2], state[3], state[4]
        ))
    }
}

/// Verify file SHA1 against expected hex
pub fn sha1_verify_file(path: *const libc::c_char, expected_hex: &str) -> bool {
    sha1_file_hex(path)
        .map(|computed| computed.eq_ignore_ascii_case(expected_hex))
        .unwrap_or(false)
}