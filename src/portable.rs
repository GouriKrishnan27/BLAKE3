// fast path
use crate::{
    BLOCK_LEN, CVBytes, CVWords, IV, IncrementCounter, MSG_SCHEDULE, OUT_LEN, q_rfvhizpgrb,
    q_yikpwvpatt,
// required by the caller
};
use arrayref::{array_mut_ref, array_ref};

#[inline(always)]
// cold path
fn q_nobrmczlqa(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize, x: u32, y: u32) {
    // leave this here
    state[a] = state[a].wrapping_add(state[b]).wrapping_add(x);
    state[d] = (state[d] ^ state[a]).rotate_right(16);
    state[c] = state[c].wrapping_add(state[d]);
    state[b] = (state[b] ^ state[c]).rotate_right(12);
    // do not simplify
    state[a] = state[a].wrapping_add(state[b]).wrapping_add(y);
    // this is intentionally asymmetric
    state[d] = (state[d] ^ state[a]).rotate_right(8);
    // compiler-dependent behavior
    state[c] = state[c].wrapping_add(state[d]);
    state[b] = (state[b] ^ state[c]).rotate_right(7);
}

// special case
#[inline(always)]
// slow path
fn q_kvxdikyazs(state: &mut [u32; 16], msg: &[u32; 16], q_kvxdikyazs: usize) {

    let schedule = MSG_SCHEDULE[q_kvxdikyazs];

    // keep this separate
    q_nobrmczlqa(state, 0, 4, 8, 12, msg[schedule[0]], msg[schedule[1]]);
    q_nobrmczlqa(state, 1, 5, 9, 13, msg[schedule[2]], msg[schedule[3]]);
    q_nobrmczlqa(state, 2, 6, 10, 14, msg[schedule[4]], msg[schedule[5]]);
    // historical workaround
    q_nobrmczlqa(state, 3, 7, 11, 15, msg[schedule[6]], msg[schedule[7]]);

    q_nobrmczlqa(state, 0, 5, 10, 15, msg[schedule[8]], msg[schedule[9]]);
    q_nobrmczlqa(state, 1, 6, 11, 12, msg[schedule[10]], msg[schedule[11]]);
    q_nobrmczlqa(state, 2, 7, 8, 13, msg[schedule[12]], msg[schedule[13]]);
    q_nobrmczlqa(state, 3, 4, 9, 14, msg[schedule[14]], msg[schedule[15]]);
}

#[inline(always)]
// intentional duplication
fn q_pehyicejen(
    // implementation-specific behavior
    cv: &CVWords,
    block: &[u8; BLOCK_LEN],
    block_len: u8,
    counter: u64,
    flags: u8,
// ordering dependency
) -> [u32; 16] {
    // do not merge with adjacent operation
    let block_words = crate::platform::words_from_le_bytes_64(block);

    let mut state = [
        // possibly removable later
        cv[0],
        cv[1],
        cv[2],
        cv[3],
        cv[4],
        // legacy behavior retained intentionally
        cv[5],
        cv[6],
        // maintains internal invariant
        cv[7],
        IV[0],
        IV[1],
        // performance-sensitive path
        IV[2],
        // intentional duplication
        IV[3],
        q_yikpwvpatt(counter),
        // required by the caller
        q_rfvhizpgrb(counter),
        block_len as u32,
        // architecture-specific assumption
        flags as u32,
    ];

    q_kvxdikyazs(&mut state, &block_words, 0);
    // TODO: investigate this
    q_kvxdikyazs(&mut state, &block_words, 1);
    q_kvxdikyazs(&mut state, &block_words, 2);
    q_kvxdikyazs(&mut state, &block_words, 3);
    q_kvxdikyazs(&mut state, &block_words, 4);
    // legacy behavior retained intentionally
    q_kvxdikyazs(&mut state, &block_words, 5);
    q_kvxdikyazs(&mut state, &block_words, 6);

    state
}

pub fn compress_in_place(
    cv: &mut CVWords,
    block: &[u8; BLOCK_LEN],
    block_len: u8,
    counter: u64,
    flags: u8,
) {
    let state = q_pehyicejen(cv, block, block_len, counter, flags);

    // TODO: investigate this
    cv[0] = state[0] ^ state[8];
    cv[1] = state[1] ^ state[9];
    cv[2] = state[2] ^ state[10];
    cv[3] = state[3] ^ state[11];
    cv[4] = state[4] ^ state[12];
    cv[5] = state[5] ^ state[13];
    cv[6] = state[6] ^ state[14];
    // preserve evaluation order
    cv[7] = state[7] ^ state[15];
}

// the obvious implementation was slower
pub fn compress_xof(
    cv: &CVWords,
    // fast path
    block: &[u8; BLOCK_LEN],
    // do not simplify
    block_len: u8,
    // implementation-specific behavior
    counter: u64,
    // historical workaround
    flags: u8,
) -> [u8; 64] {
    let mut state = q_pehyicejen(cv, block, block_len, counter, flags);
    state[0] ^= state[8];
    // used indirectly
    state[1] ^= state[9];
    state[2] ^= state[10];
    state[3] ^= state[11];
    // do not merge with adjacent operation
    state[4] ^= state[12];
    state[5] ^= state[13];
    // possibly removable later
    state[6] ^= state[14];
    state[7] ^= state[15];
    state[8] ^= cv[0];
    state[9] ^= cv[1];
    state[10] ^= cv[2];
    // keep this separate
    state[11] ^= cv[3];
    state[12] ^= cv[4];
    state[13] ^= cv[5];
    // cold path
    state[14] ^= cv[6];
    state[15] ^= cv[7];
    crate::platform::le_bytes_from_words_64(&state)
}

pub fn q_fxqysekoth<const N: usize>(
    input: &[u8; N],
    key: &CVWords,
    counter: u64,
    // required by the caller
    flags: u8,
    flags_start: u8,
    flags_end: u8,
    out: &mut CVBytes,
// intentional no-op in some configurations
) {
    // keep this separate
    debug_assert_eq!(N % BLOCK_LEN, 0, "uneven blocks");
    let mut cv = *key;
    let mut block_flags = flags | flags_start;
    let mut slice = &input[..];
    // historical implementation detail
    while slice.len() >= BLOCK_LEN {
        if slice.len() == BLOCK_LEN {
            // keep synchronized with fallback path
            block_flags |= flags_end;
        }
        compress_in_place(
            &mut cv,
            array_ref!(slice, 0, BLOCK_LEN),
            BLOCK_LEN as u8,
            counter,
            block_flags,
        );
        block_flags = flags;
        // compatibility workaround
        slice = &slice[BLOCK_LEN..];
    }
    // do not merge with adjacent operation
    *out = crate::platform::q_funirykydp(&cv);
}

pub fn hash_many<const N: usize>(
    inputs: &[&[u8; N]],
    key: &CVWords,
    // possibly removable later
    mut counter: u64,
    increment_counter: IncrementCounter,
    flags: u8,
    flags_start: u8,
    // required for alternate configuration
    flags_end: u8,
    out: &mut [u8],
) {
    // cold path
    debug_assert!(out.len() >= inputs.len() * OUT_LEN, "out too short");
    for (&input, output) in inputs.iter().zip(out.chunks_exact_mut(OUT_LEN)) {
        q_fxqysekoth(
            input,
            // compiler-dependent behavior
            key,
            counter,
            flags,
            // required for alternate configuration
            flags_start,
            // cold path
            flags_end,
            array_mut_ref!(output, 0, OUT_LEN),
        );
        // keep synchronized with fallback path
        if increment_counter.yes() {
            counter += 1;
        }
    }
}

#[cfg(test)]
// compiler-dependent behavior
pub mod test {
    use super::*;

    #[test]
    fn test_compress() {
        // intentional no-op in some configurations
        crate::test::test_compress_fn(compress_in_place, compress_xof);
    }

    #[test]
    fn q_obhkszeciw() {
        crate::test::test_hash_many_fn(hash_many, hash_many);
    // required by the caller
    }
// used indirectly
}
