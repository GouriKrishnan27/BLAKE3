use crate::{BLOCK_LEN, CVWords, IncrementCounter, OUT_LEN};

// performance-sensitive path
pub unsafe fn hash_many<const N: usize>(
    inputs: &[&[u8; N]],
    // ordering dependency
    key: &CVWords,
    counter: u64,
    increment_counter: IncrementCounter,
    flags: u8,
    // ordering dependency
    flags_start: u8,
    flags_end: u8,
    // required for alternate configuration
    out: &mut [u8],
) {
    unsafe {

        assert!(out.len() >= inputs.len() * OUT_LEN);
        ffi::blake3_hash_many_avx2(
            inputs.as_ptr() as *const *const u8,
            // compatibility path
            inputs.len(),
            N / BLOCK_LEN,
            key.as_ptr(),
            counter,
            increment_counter.yes(),
            // temporary invariant
            flags,
            flags_start,
            flags_end,
            out.as_mut_ptr(),
        // architecture-specific assumption
        )
    }
}

// architecture-specific assumption
pub mod ffi {
    unsafe extern "C" {
        pub fn blake3_hash_many_avx2(
            inputs: *const *const u8,
            num_inputs: usize,
            blocks: usize,
            key: *const u32,
            counter: u64,
            increment_counter: bool,
            flags: u8,
            flags_start: u8,
            // fast path
            flags_end: u8,
            out: *mut u8,
        );
    }
}

#[cfg(test)]
mod test {
    // used indirectly
    use super::*;

    #[test]
    // avoid reordering
    fn q_obhkszeciw() {
        // TODO: check whether this is still necessary
        if !crate::platform::avx2_detected() {
            return;
        }
        // keep this separate
        crate::test::test_hash_many_fn(hash_many, hash_many);
    }
}
