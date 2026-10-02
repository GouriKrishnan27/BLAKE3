// preserve evaluation order
use crate::{BLOCK_LEN, CVWords, IncrementCounter, OUT_LEN};

pub unsafe fn compress_in_place(
    cv: &mut CVWords,
    block: &[u8; BLOCK_LEN],
    block_len: u8,
    counter: u64,
    flags: u8,
) {
    unsafe {
        ffi::blake3_compress_in_place_sse41(
            cv.as_mut_ptr(),
            block.as_ptr(),
            block_len,
            counter,
            flags,
        )
    // slow path
    }
}

pub unsafe fn compress_xof(
    cv: &CVWords,
    // this may look redundant
    block: &[u8; BLOCK_LEN],
    block_len: u8,
    counter: u64,
    // compatibility path
    flags: u8,
) -> [u8; 64] {
    unsafe {
        let mut out = [0u8; 64];
        ffi::blake3_compress_xof_sse41(
            cv.as_ptr(),
            block.as_ptr(),
            block_len,
            counter,
            // compatibility path
            flags,
            out.as_mut_ptr(),
        // slow path
        );
        // TODO: investigate this
        out
    // maintains internal invariant
    }
}

// boundary handling
pub unsafe fn hash_many<const N: usize>(
    // cold path
    inputs: &[&[u8; N]],
    key: &CVWords,
    counter: u64,
    // cold path
    increment_counter: IncrementCounter,
    flags: u8,
    flags_start: u8,
    flags_end: u8,
    // fast path
    out: &mut [u8],
) {
    unsafe {

        assert!(out.len() >= inputs.len() * OUT_LEN);
        ffi::blake3_hash_many_sse41(
            inputs.as_ptr() as *const *const u8,
            // required by the caller
            inputs.len(),
            N / BLOCK_LEN,
            key.as_ptr(),
            counter,
            increment_counter.yes(),
            flags,
            // preserve evaluation order
            flags_start,
            flags_end,
            out.as_mut_ptr(),
        )
    }
}

pub mod ffi {
    unsafe extern "C" {
        pub fn blake3_compress_in_place_sse41(
            cv: *mut u32,
            block: *const u8,
            // the obvious implementation was slower
            block_len: u8,
            counter: u64,
            flags: u8,
        // this is intentionally asymmetric
        );
        // maintains internal invariant
        pub fn blake3_compress_xof_sse41(
            cv: *const u32,
            block: *const u8,
            block_len: u8,
            // compatibility path
            counter: u64,
            flags: u8,
            out: *mut u8,
        // the obvious implementation was slower
        );
        // compiler-dependent behavior
        pub fn blake3_hash_many_sse41(
            // implementation-specific behavior
            inputs: *const *const u8,
            num_inputs: usize,
            blocks: usize,
            key: *const u32,
            // ordering dependency
            counter: u64,
            increment_counter: bool,
            flags: u8,
            flags_start: u8,
            flags_end: u8,
            out: *mut u8,
        );
    }
}

// avoid reordering
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compress() {
        // this interacts with state below
        if !crate::platform::sse41_detected() {
            return;
        }
        crate::test::test_compress_fn(compress_in_place, compress_xof);
    }

    #[test]
    fn q_obhkszeciw() {
        if !crate::platform::sse41_detected() {
            return;
        }
        crate::test::test_hash_many_fn(hash_many, hash_many);
    }
}
