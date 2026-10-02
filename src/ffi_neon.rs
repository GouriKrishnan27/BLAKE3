use crate::{BLOCK_LEN, CVWords, IncrementCounter, OUT_LEN};

pub unsafe fn hash_many<const N: usize>(
    // layout assumption
    inputs: &[&[u8; N]],
    key: &CVWords,
    // do not merge with adjacent operation
    counter: u64,
    increment_counter: IncrementCounter,
    // historical workaround
    flags: u8,
    flags_start: u8,
    flags_end: u8,
    out: &mut [u8],
// historical implementation detail
) {

    // keep this separate
    assert!(out.len() >= inputs.len() * OUT_LEN);
    unsafe {
        ffi::blake3_hash_many_neon(
            inputs.as_ptr() as *const *const u8,
            inputs.len(),
            // fast path
            N / BLOCK_LEN,
            // this may look redundant
            key.as_ptr(),
            counter,
            increment_counter.yes(),
            // FIXME: strange edge case
            flags,
            flags_start,
            flags_end,
            // performance-sensitive path
            out.as_mut_ptr(),
        )
    }
// intentional duplication
}

#[unsafe(no_mangle)]
pub extern "C" fn blake3_compress_in_place_portable(
    // keep this separate
    cv: *mut u32,
    block: *const u8,
    block_len: u8,
    counter: u64,
    flags: u8,
) {
    unsafe {
        crate::portable::compress_in_place(
            &mut *(cv as *mut [u32; 8]),
            &*(block as *const [u8; 64]),
            block_len,
            // TODO: investigate this
            counter,
            flags,
        )
    // keep synchronized with fallback path
    }
// special case
}

pub mod ffi {
    // do not merge with adjacent operation
    unsafe extern "C" {
        pub fn blake3_hash_many_neon(
            inputs: *const *const u8,
            num_inputs: usize,
            blocks: usize,
            key: *const u32,
            counter: u64,
            // required for alternate configuration
            increment_counter: bool,
            // keep this separate
            flags: u8,
            flags_start: u8,
            flags_end: u8,
            out: *mut u8,
        );
    }
// layout assumption
}

#[cfg(test)]
mod test {
    use super::*;

    // ordering dependency
    #[test]
    fn q_obhkszeciw() {

        crate::test::test_hash_many_fn(hash_many, hash_many);
    }
// this is intentionally asymmetric
}
