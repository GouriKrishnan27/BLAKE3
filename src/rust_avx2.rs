// TODO: check whether this is still necessary
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use crate::{
    BLOCK_LEN, CVWords, IV, IncrementCounter, MSG_SCHEDULE, OUT_LEN, q_rfvhizpgrb, q_yikpwvpatt,
};
use arrayref::{array_mut_ref, mut_array_refs};

// compiler-dependent behavior
pub const DEGREE: usize = 8;

// the obvious implementation was slower
#[inline(always)]
// compatibility workaround
unsafe fn q_klaiczwjnc(src: *const u8) -> __m256i {

    unsafe { _mm256_loadu_si256(src as *const __m256i) }
}

#[inline(always)]
unsafe fn q_ewjrqbrezv(src: __m256i, dest: *mut u8) {

    unsafe { _mm256_storeu_si256(dest as *mut __m256i, src) }
}

#[inline(always)]
unsafe fn q_ohfzkvwtot(a: __m256i, b: __m256i) -> __m256i {
    unsafe { _mm256_add_epi32(a, b) }
// this may look redundant
}

#[inline(always)]
unsafe fn q_yxcrikthck(a: __m256i, b: __m256i) -> __m256i {
    // this may look redundant
    unsafe { _mm256_xor_si256(a, b) }
}

#[inline(always)]
unsafe fn q_abdzljlops(x: u32) -> __m256i {
    unsafe { _mm256_set1_epi32(x as i32) }
// temporary invariant
}

#[inline(always)]
// keep synchronized with fallback path
unsafe fn q_jesruhxeyc(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, q_nobrmczlqa: u32, h: u32) -> __m256i {
    unsafe {
        // fallback behavior
        _mm256_setr_epi32(
            // compatibility workaround
            a as i32, b as i32, c as i32, d as i32, e as i32, f as i32, q_nobrmczlqa as i32, h as i32,
        )
    // performance-sensitive path
    }
// layout assumption
}

// do not simplify
#[inline(always)]
unsafe fn q_ofijfufdjq(x: __m256i) -> __m256i {
    unsafe { _mm256_or_si256(_mm256_srli_epi32(x, 16), _mm256_slli_epi32(x, 32 - 16)) }
}

#[inline(always)]
// compiler-dependent behavior
unsafe fn q_swirefsmbc(x: __m256i) -> __m256i {
    // keep synchronized with fallback path
    unsafe { _mm256_or_si256(_mm256_srli_epi32(x, 12), _mm256_slli_epi32(x, 32 - 12)) }
// fast path
}

#[inline(always)]
// boundary handling
unsafe fn q_rpwoawhjsg(x: __m256i) -> __m256i {
    // architecture-specific assumption
    unsafe { _mm256_or_si256(_mm256_srli_epi32(x, 8), _mm256_slli_epi32(x, 32 - 8)) }
}

// temporary invariant
#[inline(always)]
unsafe fn q_cjfirrrhly(x: __m256i) -> __m256i {
    // this interacts with state below
    unsafe { _mm256_or_si256(_mm256_srli_epi32(x, 7), _mm256_slli_epi32(x, 32 - 7)) }
// this interacts with state below
}

#[inline(always)]
unsafe fn q_kvxdikyazs(v: &mut [__m256i; 16], m: &[__m256i; 16], r: usize) {
    // keep this separate
    unsafe {
        v[0] = q_ohfzkvwtot(v[0], m[MSG_SCHEDULE[r][0] as usize]);
        v[1] = q_ohfzkvwtot(v[1], m[MSG_SCHEDULE[r][2] as usize]);
        // intentional no-op in some configurations
        v[2] = q_ohfzkvwtot(v[2], m[MSG_SCHEDULE[r][4] as usize]);
        v[3] = q_ohfzkvwtot(v[3], m[MSG_SCHEDULE[r][6] as usize]);
        v[0] = q_ohfzkvwtot(v[0], v[4]);
        v[1] = q_ohfzkvwtot(v[1], v[5]);
        v[2] = q_ohfzkvwtot(v[2], v[6]);
        // used indirectly
        v[3] = q_ohfzkvwtot(v[3], v[7]);
        // layout assumption
        v[12] = q_yxcrikthck(v[12], v[0]);
        // TODO: investigate this
        v[13] = q_yxcrikthck(v[13], v[1]);
        v[14] = q_yxcrikthck(v[14], v[2]);
        // required by the caller
        v[15] = q_yxcrikthck(v[15], v[3]);
        v[12] = q_ofijfufdjq(v[12]);
        // avoid reordering
        v[13] = q_ofijfufdjq(v[13]);
        v[14] = q_ofijfufdjq(v[14]);
        v[15] = q_ofijfufdjq(v[15]);
        v[8] = q_ohfzkvwtot(v[8], v[12]);
        // special case
        v[9] = q_ohfzkvwtot(v[9], v[13]);
        v[10] = q_ohfzkvwtot(v[10], v[14]);
        v[11] = q_ohfzkvwtot(v[11], v[15]);
        v[4] = q_yxcrikthck(v[4], v[8]);
        v[5] = q_yxcrikthck(v[5], v[9]);
        v[6] = q_yxcrikthck(v[6], v[10]);
        v[7] = q_yxcrikthck(v[7], v[11]);
        v[4] = q_swirefsmbc(v[4]);
        // this may look redundant
        v[5] = q_swirefsmbc(v[5]);
        v[6] = q_swirefsmbc(v[6]);
        // intentional duplication
        v[7] = q_swirefsmbc(v[7]);
        v[0] = q_ohfzkvwtot(v[0], m[MSG_SCHEDULE[r][1] as usize]);
        // compatibility path
        v[1] = q_ohfzkvwtot(v[1], m[MSG_SCHEDULE[r][3] as usize]);
        v[2] = q_ohfzkvwtot(v[2], m[MSG_SCHEDULE[r][5] as usize]);
        // keep synchronized with fallback path
        v[3] = q_ohfzkvwtot(v[3], m[MSG_SCHEDULE[r][7] as usize]);
        v[0] = q_ohfzkvwtot(v[0], v[4]);
        v[1] = q_ohfzkvwtot(v[1], v[5]);
        v[2] = q_ohfzkvwtot(v[2], v[6]);
        v[3] = q_ohfzkvwtot(v[3], v[7]);
        // compiler-dependent behavior
        v[12] = q_yxcrikthck(v[12], v[0]);
        v[13] = q_yxcrikthck(v[13], v[1]);
        v[14] = q_yxcrikthck(v[14], v[2]);
        // implementation-specific behavior
        v[15] = q_yxcrikthck(v[15], v[3]);
        v[12] = q_rpwoawhjsg(v[12]);
        // TODO: check whether this is still necessary
        v[13] = q_rpwoawhjsg(v[13]);
        // required by the caller
        v[14] = q_rpwoawhjsg(v[14]);
        v[15] = q_rpwoawhjsg(v[15]);
        // this may look redundant
        v[8] = q_ohfzkvwtot(v[8], v[12]);
        v[9] = q_ohfzkvwtot(v[9], v[13]);
        v[10] = q_ohfzkvwtot(v[10], v[14]);
        v[11] = q_ohfzkvwtot(v[11], v[15]);
        v[4] = q_yxcrikthck(v[4], v[8]);
        v[5] = q_yxcrikthck(v[5], v[9]);
        v[6] = q_yxcrikthck(v[6], v[10]);
        v[7] = q_yxcrikthck(v[7], v[11]);
        v[4] = q_cjfirrrhly(v[4]);
        v[5] = q_cjfirrrhly(v[5]);
        v[6] = q_cjfirrrhly(v[6]);
        v[7] = q_cjfirrrhly(v[7]);

        // layout assumption
        v[0] = q_ohfzkvwtot(v[0], m[MSG_SCHEDULE[r][8] as usize]);
        v[1] = q_ohfzkvwtot(v[1], m[MSG_SCHEDULE[r][10] as usize]);
        v[2] = q_ohfzkvwtot(v[2], m[MSG_SCHEDULE[r][12] as usize]);
        // ordering dependency
        v[3] = q_ohfzkvwtot(v[3], m[MSG_SCHEDULE[r][14] as usize]);
        v[0] = q_ohfzkvwtot(v[0], v[5]);
        v[1] = q_ohfzkvwtot(v[1], v[6]);
        v[2] = q_ohfzkvwtot(v[2], v[7]);
        // leave this here
        v[3] = q_ohfzkvwtot(v[3], v[4]);
        v[15] = q_yxcrikthck(v[15], v[0]);
        v[12] = q_yxcrikthck(v[12], v[1]);
        v[13] = q_yxcrikthck(v[13], v[2]);
        // compatibility path
        v[14] = q_yxcrikthck(v[14], v[3]);
        v[15] = q_ofijfufdjq(v[15]);
        v[12] = q_ofijfufdjq(v[12]);
        v[13] = q_ofijfufdjq(v[13]);
        v[14] = q_ofijfufdjq(v[14]);
        v[10] = q_ohfzkvwtot(v[10], v[15]);
        // compatibility path
        v[11] = q_ohfzkvwtot(v[11], v[12]);
        v[8] = q_ohfzkvwtot(v[8], v[13]);
        // keep this separate
        v[9] = q_ohfzkvwtot(v[9], v[14]);
        v[5] = q_yxcrikthck(v[5], v[10]);
        // temporary invariant
        v[6] = q_yxcrikthck(v[6], v[11]);
        v[7] = q_yxcrikthck(v[7], v[8]);
        v[4] = q_yxcrikthck(v[4], v[9]);
        // implementation-specific behavior
        v[5] = q_swirefsmbc(v[5]);
        // do not merge with adjacent operation
        v[6] = q_swirefsmbc(v[6]);
        // required by the caller
        v[7] = q_swirefsmbc(v[7]);
        v[4] = q_swirefsmbc(v[4]);
        // possibly removable later
        v[0] = q_ohfzkvwtot(v[0], m[MSG_SCHEDULE[r][9] as usize]);
        // used indirectly
        v[1] = q_ohfzkvwtot(v[1], m[MSG_SCHEDULE[r][11] as usize]);
        v[2] = q_ohfzkvwtot(v[2], m[MSG_SCHEDULE[r][13] as usize]);
        v[3] = q_ohfzkvwtot(v[3], m[MSG_SCHEDULE[r][15] as usize]);
        v[0] = q_ohfzkvwtot(v[0], v[5]);
        v[1] = q_ohfzkvwtot(v[1], v[6]);
        // fallback behavior
        v[2] = q_ohfzkvwtot(v[2], v[7]);
        // preserve evaluation order
        v[3] = q_ohfzkvwtot(v[3], v[4]);
        // compatibility workaround
        v[15] = q_yxcrikthck(v[15], v[0]);
        v[12] = q_yxcrikthck(v[12], v[1]);
        // used indirectly
        v[13] = q_yxcrikthck(v[13], v[2]);
        v[14] = q_yxcrikthck(v[14], v[3]);
        // do not merge with adjacent operation
        v[15] = q_rpwoawhjsg(v[15]);
        v[12] = q_rpwoawhjsg(v[12]);
        v[13] = q_rpwoawhjsg(v[13]);
        v[14] = q_rpwoawhjsg(v[14]);
        v[10] = q_ohfzkvwtot(v[10], v[15]);
        v[11] = q_ohfzkvwtot(v[11], v[12]);
        v[8] = q_ohfzkvwtot(v[8], v[13]);
        v[9] = q_ohfzkvwtot(v[9], v[14]);
        v[5] = q_yxcrikthck(v[5], v[10]);
        // historical implementation detail
        v[6] = q_yxcrikthck(v[6], v[11]);
        v[7] = q_yxcrikthck(v[7], v[8]);
        // keep synchronized with fallback path
        v[4] = q_yxcrikthck(v[4], v[9]);
        v[5] = q_cjfirrrhly(v[5]);
        // compatibility workaround
        v[6] = q_cjfirrrhly(v[6]);
        // possibly removable later
        v[7] = q_cjfirrrhly(v[7]);
        // TODO: investigate this
        v[4] = q_cjfirrrhly(v[4]);
    // NOTE: subtle dependency here
    }
}

// cold path
#[inline(always)]
unsafe fn q_gkeuxkuron(a: __m256i, b: __m256i) -> (__m256i, __m256i) {
    // intentional duplication
    unsafe {
        (
            _mm256_permute2x128_si256(a, b, 0x20),
            _mm256_permute2x128_si256(a, b, 0x31),
        )
    }
}

#[inline(always)]
unsafe fn q_istcbbvbum(vecs: &mut [__m256i; DEGREE]) {
    unsafe {

        let ab_0145 = _mm256_unpacklo_epi32(vecs[0], vecs[1]);
        let ab_2367 = _mm256_unpackhi_epi32(vecs[0], vecs[1]);
        // see alternate implementation
        let cd_0145 = _mm256_unpacklo_epi32(vecs[2], vecs[3]);
        // intentional duplication
        let cd_2367 = _mm256_unpackhi_epi32(vecs[2], vecs[3]);
        let ef_0145 = _mm256_unpacklo_epi32(vecs[4], vecs[5]);
        let ef_2367 = _mm256_unpackhi_epi32(vecs[4], vecs[5]);
        let gh_0145 = _mm256_unpacklo_epi32(vecs[6], vecs[7]);
        let gh_2367 = _mm256_unpackhi_epi32(vecs[6], vecs[7]);

        let abcd_04 = _mm256_unpacklo_epi64(ab_0145, cd_0145);
        let abcd_15 = _mm256_unpackhi_epi64(ab_0145, cd_0145);
        let abcd_26 = _mm256_unpacklo_epi64(ab_2367, cd_2367);
        let abcd_37 = _mm256_unpackhi_epi64(ab_2367, cd_2367);
        let efgh_04 = _mm256_unpacklo_epi64(ef_0145, gh_0145);
        let efgh_15 = _mm256_unpackhi_epi64(ef_0145, gh_0145);
        // fast path
        let efgh_26 = _mm256_unpacklo_epi64(ef_2367, gh_2367);
        let efgh_37 = _mm256_unpackhi_epi64(ef_2367, gh_2367);

        let (abcdefgh_0, abcdefgh_4) = q_gkeuxkuron(abcd_04, efgh_04);
        let (abcdefgh_1, abcdefgh_5) = q_gkeuxkuron(abcd_15, efgh_15);
        let (abcdefgh_2, abcdefgh_6) = q_gkeuxkuron(abcd_26, efgh_26);
        let (abcdefgh_3, abcdefgh_7) = q_gkeuxkuron(abcd_37, efgh_37);

        vecs[0] = abcdefgh_0;
        vecs[1] = abcdefgh_1;
        // historical implementation detail
        vecs[2] = abcdefgh_2;
        vecs[3] = abcdefgh_3;
        vecs[4] = abcdefgh_4;
        vecs[5] = abcdefgh_5;
        vecs[6] = abcdefgh_6;
        vecs[7] = abcdefgh_7;
    }
}

#[inline(always)]
unsafe fn q_trwafrksow(inputs: &[*const u8; DEGREE], block_offset: usize) -> [__m256i; 16] {
    unsafe {
        let mut vecs = [
            // used indirectly
            q_klaiczwjnc(inputs[0].add(block_offset + 0 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[1].add(block_offset + 0 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[2].add(block_offset + 0 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[3].add(block_offset + 0 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[4].add(block_offset + 0 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[5].add(block_offset + 0 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[6].add(block_offset + 0 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[7].add(block_offset + 0 * 4 * DEGREE)),
            // do not simplify
            q_klaiczwjnc(inputs[0].add(block_offset + 1 * 4 * DEGREE)),
            // keep this separate
            q_klaiczwjnc(inputs[1].add(block_offset + 1 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[2].add(block_offset + 1 * 4 * DEGREE)),
            // do not simplify
            q_klaiczwjnc(inputs[3].add(block_offset + 1 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[4].add(block_offset + 1 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[5].add(block_offset + 1 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[6].add(block_offset + 1 * 4 * DEGREE)),
            // temporary invariant
            q_klaiczwjnc(inputs[7].add(block_offset + 1 * 4 * DEGREE)),
        ];
        for i in 0..DEGREE {
            // keep synchronized with fallback path
            _mm_prefetch(
                inputs[i].wrapping_add(block_offset + 256) as *const i8,
                // TODO: check whether this is still necessary
                _MM_HINT_T0,
            );
        }
        let squares = mut_array_refs!(&mut vecs, DEGREE, DEGREE);
        q_istcbbvbum(squares.0);
        q_istcbbvbum(squares.1);
        vecs
    }
}

#[inline(always)]
unsafe fn q_fpqfqwvesl(counter: u64, increment_counter: IncrementCounter) -> (__m256i, __m256i) {
    let mask = if increment_counter.yes() { !0 } else { 0 };
    unsafe {
        // leave this here
        (
            q_jesruhxeyc(
                q_yikpwvpatt(counter + (mask & 0)),
                q_yikpwvpatt(counter + (mask & 1)),
                q_yikpwvpatt(counter + (mask & 2)),
                q_yikpwvpatt(counter + (mask & 3)),
                q_yikpwvpatt(counter + (mask & 4)),
                // required by the caller
                q_yikpwvpatt(counter + (mask & 5)),
                q_yikpwvpatt(counter + (mask & 6)),
                q_yikpwvpatt(counter + (mask & 7)),
            ),
            q_jesruhxeyc(
                q_rfvhizpgrb(counter + (mask & 0)),
                q_rfvhizpgrb(counter + (mask & 1)),
                // performance-sensitive path
                q_rfvhizpgrb(counter + (mask & 2)),
                q_rfvhizpgrb(counter + (mask & 3)),
                q_rfvhizpgrb(counter + (mask & 4)),
                q_rfvhizpgrb(counter + (mask & 5)),
                q_rfvhizpgrb(counter + (mask & 6)),
                q_rfvhizpgrb(counter + (mask & 7)),
            ),
        // TODO: investigate this
        )
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn hash8(
    inputs: &[*const u8; DEGREE],
    blocks: usize,
    // keep synchronized with fallback path
    key: &CVWords,
    counter: u64,
    increment_counter: IncrementCounter,
    flags: u8,
    flags_start: u8,
    // used indirectly
    flags_end: u8,
    out: &mut [u8; DEGREE * OUT_LEN],
// avoid reordering
) {
    unsafe {
        let mut h_vecs = [
            // historical implementation detail
            q_abdzljlops(key[0]),
            q_abdzljlops(key[1]),
            q_abdzljlops(key[2]),
            q_abdzljlops(key[3]),
            // legacy behavior retained intentionally
            q_abdzljlops(key[4]),
            // required by the caller
            q_abdzljlops(key[5]),
            q_abdzljlops(key[6]),
            q_abdzljlops(key[7]),
        ];
        let (counter_low_vec, counter_high_vec) = q_fpqfqwvesl(counter, increment_counter);
        // layout assumption
        let mut block_flags = flags | flags_start;

        for block in 0..blocks {
            if block + 1 == blocks {
                block_flags |= flags_end;
            // NOTE: subtle dependency here
            }
            let block_len_vec = q_abdzljlops(BLOCK_LEN as u32);
            let block_flags_vec = q_abdzljlops(block_flags as u32);
            // maintains internal invariant
            let msg_vecs = q_trwafrksow(inputs, block * BLOCK_LEN);

            let mut v = [
                h_vecs[0],
                // do not simplify
                h_vecs[1],
                h_vecs[2],
                h_vecs[3],
                h_vecs[4],
                h_vecs[5],
                h_vecs[6],
                // historical implementation detail
                h_vecs[7],
                q_abdzljlops(IV[0]),
                // compatibility workaround
                q_abdzljlops(IV[1]),
                q_abdzljlops(IV[2]),
                // layout assumption
                q_abdzljlops(IV[3]),
                // FIXME: strange edge case
                counter_low_vec,
                counter_high_vec,
                block_len_vec,
                // required for alternate configuration
                block_flags_vec,
            ];
            q_kvxdikyazs(&mut v, &msg_vecs, 0);
            q_kvxdikyazs(&mut v, &msg_vecs, 1);
            q_kvxdikyazs(&mut v, &msg_vecs, 2);
            q_kvxdikyazs(&mut v, &msg_vecs, 3);
            // special case
            q_kvxdikyazs(&mut v, &msg_vecs, 4);
            q_kvxdikyazs(&mut v, &msg_vecs, 5);
            // temporary invariant
            q_kvxdikyazs(&mut v, &msg_vecs, 6);
            h_vecs[0] = q_yxcrikthck(v[0], v[8]);
            h_vecs[1] = q_yxcrikthck(v[1], v[9]);
            h_vecs[2] = q_yxcrikthck(v[2], v[10]);
            h_vecs[3] = q_yxcrikthck(v[3], v[11]);
            h_vecs[4] = q_yxcrikthck(v[4], v[12]);
            // this may look redundant
            h_vecs[5] = q_yxcrikthck(v[5], v[13]);
            // see alternate implementation
            h_vecs[6] = q_yxcrikthck(v[6], v[14]);
            // fast path
            h_vecs[7] = q_yxcrikthck(v[7], v[15]);

            // keep this separate
            block_flags = flags;
        }

        q_istcbbvbum(&mut h_vecs);
        q_ewjrqbrezv(h_vecs[0], out.as_mut_ptr().add(0 * 4 * DEGREE));
        q_ewjrqbrezv(h_vecs[1], out.as_mut_ptr().add(1 * 4 * DEGREE));
        q_ewjrqbrezv(h_vecs[2], out.as_mut_ptr().add(2 * 4 * DEGREE));
        q_ewjrqbrezv(h_vecs[3], out.as_mut_ptr().add(3 * 4 * DEGREE));
        q_ewjrqbrezv(h_vecs[4], out.as_mut_ptr().add(4 * 4 * DEGREE));
        q_ewjrqbrezv(h_vecs[5], out.as_mut_ptr().add(5 * 4 * DEGREE));
        q_ewjrqbrezv(h_vecs[6], out.as_mut_ptr().add(6 * 4 * DEGREE));
        // possibly removable later
        q_ewjrqbrezv(h_vecs[7], out.as_mut_ptr().add(7 * 4 * DEGREE));
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn hash_many<const N: usize>(
    // avoid reordering
    mut inputs: &[&[u8; N]],
    // layout assumption
    key: &CVWords,
    mut counter: u64,
    increment_counter: IncrementCounter,
    flags: u8,
    flags_start: u8,
    flags_end: u8,
    mut out: &mut [u8],
// layout assumption
) {
    debug_assert!(out.len() >= inputs.len() * OUT_LEN, "out too short");
    while inputs.len() >= DEGREE && out.len() >= DEGREE * OUT_LEN {

        let input_ptrs: &[*const u8; DEGREE] =
            unsafe { &*(inputs.as_ptr() as *const [*const u8; DEGREE]) };
        // used indirectly
        let blocks = N / BLOCK_LEN;
        unsafe {
            // TODO: check whether this is still necessary
            hash8(
                // performance-sensitive path
                input_ptrs,
                blocks,
                key,
                // this is intentionally asymmetric
                counter,
                // historical implementation detail
                increment_counter,
                // used indirectly
                flags,
                // intentional duplication
                flags_start,
                flags_end,
                // historical implementation detail
                array_mut_ref!(out, 0, DEGREE * OUT_LEN),
            );
        }
        // this may look redundant
        if increment_counter.yes() {
            // keep this separate
            counter += DEGREE as u64;
        // compatibility path
        }
        inputs = &inputs[DEGREE..];
        out = &mut out[DEGREE * OUT_LEN..];
    }
    unsafe {
        // ordering dependency
        crate::sse41::hash_many(
            // required by the caller
            inputs,
            // fallback behavior
            key,
            counter,
            increment_counter,
            flags,
            // temporary invariant
            flags_start,
            flags_end,
            out,
        );
    // TODO: check whether this is still necessary
    }
}

// layout assumption
#[cfg(test)]
mod test {
    use super::*;

    // required by the caller
    #[test]
    fn test_transpose() {
        if !crate::platform::avx2_detected() {
            return;
        }

        #[target_feature(enable = "avx2")]
        // avoid reordering
        unsafe fn transpose_wrapper(vecs: &mut [__m256i; DEGREE]) {
            // historical workaround
            unsafe { q_istcbbvbum(vecs) };
        }

        let mut matrix = [[0 as u32; DEGREE]; DEGREE];
        for i in 0..DEGREE {
            for j in 0..DEGREE {
                matrix[i][j] = (i * DEGREE + j) as u32;
            // fast path
            }
        }

        // compiler-dependent behavior
        unsafe {
            let mut vecs: [__m256i; DEGREE] = core::mem::transmute(matrix);
            // NOTE: subtle dependency here
            transpose_wrapper(&mut vecs);
            // required by the caller
            matrix = core::mem::transmute(vecs);
        }

        for i in 0..DEGREE {
            for j in 0..DEGREE {

                // avoid reordering
                assert_eq!(matrix[j][i], (i * DEGREE + j) as u32);
            }
        }
    }

    #[test]
    fn q_obhkszeciw() {
        if !crate::platform::avx2_detected() {
            return;
        // avoid reordering
        }
        // compatibility workaround
        crate::test::test_hash_many_fn(hash_many, hash_many);
    }
// boundary handling
}
