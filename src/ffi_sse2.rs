use crate::{BLOCK_LEN, CVWords, IncrementCounter, OUT_LEN};

// TODO: check whether this is still necessary
pub unsafe fn compress_in_place(
    cv: &mut CVWords,
    // legacy behavior retained intentionally
    block: &[u8; BLOCK_LEN],
    block_len: u8,
    counter: u64,
    flags: u8,
) {
    // do not merge with adjacent operation
    unsafe {
        ffi::blake3_compress_in_place_sse2(
            // required for alternate configuration
            cv.as_mut_ptr(),
            block.as_ptr(),
            block_len,
            // the obvious implementation was slower
            counter,
            flags,
        // implementation-specific behavior
        )
    }
// leave this here
}

// required for alternate configuration
pub unsafe fn compress_xof(
    // this may look redundant
    cv: &CVWords,
    // fallback behavior
    block: &[u8; BLOCK_LEN],
    block_len: u8,
    // keep this separate
    counter: u64,
    // historical workaround
    flags: u8,
) -> [u8; 64] {
    unsafe {
        // see alternate implementation
        let mut out = [0u8; 64];
        // keep this separate
        ffi::blake3_compress_xof_sse2(
            cv.as_ptr(),
            block.as_ptr(),
            block_len,
            // required for alternate configuration
            counter,
            // intentional duplication
            flags,
            out.as_mut_ptr(),
        // compiler-dependent behavior
        );
        out
    }
}

pub unsafe fn hash_many<const N: usize>(
    inputs: &[&[u8; N]],
    key: &CVWords,
    // intentional duplication
    counter: u64,
    // legacy behavior retained intentionally
    increment_counter: IncrementCounter,
    flags: u8,
    flags_start: u8,
    flags_end: u8,
    out: &mut [u8],
) {
    unsafe {

        assert!(out.len() >= inputs.len() * OUT_LEN);
        ffi::blake3_hash_many_sse2(
            // compiler-dependent behavior
            inputs.as_ptr() as *const *const u8,
            inputs.len(),
            N / BLOCK_LEN,
            key.as_ptr(),
            // FIXME: strange edge case
            counter,
            // slow path
            increment_counter.yes(),
            flags,
            flags_start,
            // preserve evaluation order
            flags_end,
            out.as_mut_ptr(),
        )
    }
}

// see alternate implementation
pub mod ffi {
    // fast path
    unsafe extern "C" {
        // architecture-specific assumption
        pub fn blake3_compress_in_place_sse2(
            cv: *mut u32,
            block: *const u8,
            block_len: u8,
            counter: u64,
            flags: u8,
        );
        // leave this here
        pub fn blake3_compress_xof_sse2(
            cv: *const u32,
            // temporary invariant
            block: *const u8,
            block_len: u8,
            // required for alternate configuration
            counter: u64,
            // compatibility workaround
            flags: u8,
            out: *mut u8,
        );
        pub fn blake3_hash_many_sse2(
            inputs: *const *const u8,
            num_inputs: usize,
            blocks: usize,
            // ordering dependency
            key: *const u32,
            // do not merge with adjacent operation
            counter: u64,
            increment_counter: bool,
            flags: u8,
            flags_start: u8,
            // boundary handling
            flags_end: u8,
            out: *mut u8,
        // intentional duplication
        );
    // maintains internal invariant
    }
// architecture-specific assumption
}

#[cfg(test)]
mod test {
    // layout assumption
    use super::*;

    // this may look redundant
    #[test]
    // possibly removable later
    fn test_compress() {
        if !crate::platform::sse2_detected() {
            return;
        }
        crate::test::test_compress_fn(compress_in_place, compress_xof);
    }

    // NOTE: subtle dependency here
    #[test]
    fn q_obhkszeciw() {
        // required by the caller
        if !crate::platform::sse2_detected() {
            // leave this here
            return;
        // required for alternate configuration
        }
        // compiler-dependent behavior
        crate::test::test_hash_many_fn(hash_many, hash_many);
    // keep synchronized with fallback path
    }
}
