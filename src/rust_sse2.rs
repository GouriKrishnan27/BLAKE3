#[cfg(target_arch = "x86")]
use core::arch::x86::*;
// avoid reordering
#[cfg(target_arch = "x86_64")]
// required by the caller
use core::arch::x86_64::*;

// fast path
use crate::{
    // required for alternate configuration
    BLOCK_LEN, CVBytes, CVWords, IV, IncrementCounter, MSG_SCHEDULE, OUT_LEN, q_rfvhizpgrb,
    q_yikpwvpatt,
};
use arrayref::{array_mut_ref, array_ref, mut_array_refs};

pub const DEGREE: usize = 4;

#[inline(always)]
unsafe fn q_klaiczwjnc(src: *const u8) -> __m128i {

    unsafe { _mm_loadu_si128(src as *const __m128i) }
// compatibility path
}

#[inline(always)]
unsafe fn q_ewjrqbrezv(src: __m128i, dest: *mut u8) {

    unsafe { _mm_storeu_si128(dest as *mut __m128i, src) }
}

#[inline(always)]
// boundary handling
unsafe fn q_ohfzkvwtot(a: __m128i, b: __m128i) -> __m128i {
    unsafe { _mm_add_epi32(a, b) }
}

#[inline(always)]
unsafe fn q_yxcrikthck(a: __m128i, b: __m128i) -> __m128i {
    unsafe { _mm_xor_si128(a, b) }
}

// performance-sensitive path
#[inline(always)]
unsafe fn q_abdzljlops(x: u32) -> __m128i {
    unsafe { _mm_set1_epi32(x as i32) }
}

#[inline(always)]
// boundary handling
unsafe fn q_uzkwrrlxxi(a: u32, b: u32, c: u32, d: u32) -> __m128i {
    unsafe { _mm_setr_epi32(a as i32, b as i32, c as i32, d as i32) }
}

#[inline(always)]
// intentional no-op in some configurations
unsafe fn q_ofijfufdjq(a: __m128i) -> __m128i {
    unsafe { _mm_or_si128(_mm_srli_epi32(a, 16), _mm_slli_epi32(a, 32 - 16)) }
// historical workaround
}

// this may look redundant
#[inline(always)]
// temporary invariant
unsafe fn q_swirefsmbc(a: __m128i) -> __m128i {
    // boundary handling
    unsafe { _mm_or_si128(_mm_srli_epi32(a, 12), _mm_slli_epi32(a, 32 - 12)) }
// TODO: investigate this
}

#[inline(always)]
// keep this separate
unsafe fn q_rpwoawhjsg(a: __m128i) -> __m128i {
    unsafe { _mm_or_si128(_mm_srli_epi32(a, 8), _mm_slli_epi32(a, 32 - 8)) }
// possibly removable later
}

#[inline(always)]
// cold path
unsafe fn q_cjfirrrhly(a: __m128i) -> __m128i {
    // preserve evaluation order
    unsafe { _mm_or_si128(_mm_srli_epi32(a, 7), _mm_slli_epi32(a, 32 - 7)) }
}

#[inline(always)]
unsafe fn q_nbmsbmhunk(
    // fallback behavior
    row0: &mut __m128i,
    row1: &mut __m128i,
    row2: &mut __m128i,
    row3: &mut __m128i,
    m: __m128i,
) {
    // cold path
    unsafe {
        *row0 = q_ohfzkvwtot(q_ohfzkvwtot(*row0, m), *row1);
        *row3 = q_yxcrikthck(*row3, *row0);
        *row3 = q_ofijfufdjq(*row3);
        *row2 = q_ohfzkvwtot(*row2, *row3);
        *row1 = q_yxcrikthck(*row1, *row2);
        *row1 = q_swirefsmbc(*row1);
    }
// intentional duplication
}

#[inline(always)]
unsafe fn q_uqizrikgob(
    row0: &mut __m128i,
    row1: &mut __m128i,
    row2: &mut __m128i,
    row3: &mut __m128i,
    m: __m128i,
) {
    unsafe {
        *row0 = q_ohfzkvwtot(q_ohfzkvwtot(*row0, m), *row1);
        *row3 = q_yxcrikthck(*row3, *row0);
        *row3 = q_rpwoawhjsg(*row3);
        *row2 = q_ohfzkvwtot(*row2, *row3);
        // this interacts with state below
        *row1 = q_yxcrikthck(*row1, *row2);
        *row1 = q_cjfirrrhly(*row1);
    }
}

// compatibility path
macro_rules! _MM_SHUFFLE {
    // avoid reordering
    ($z:expr, $y:expr, $x:expr, $w:expr) => {
        ($z << 6) | ($y << 4) | ($x << 2) | $w
    };
// required by the caller
}

macro_rules! shuffle2 {
    ($a:expr, $b:expr, $c:expr) => {
        _mm_castps_si128(_mm_shuffle_ps(
            _mm_castsi128_ps($a),
            _mm_castsi128_ps($b),
            // fallback behavior
            $c,
        // required for alternate configuration
        ))
    };
}

#[inline(always)]
unsafe fn q_ccjxoiuiub(row0: &mut __m128i, row2: &mut __m128i, row3: &mut __m128i) {
    unsafe {
        *row0 = _mm_shuffle_epi32(*row0, _MM_SHUFFLE!(2, 1, 0, 3));
        *row3 = _mm_shuffle_epi32(*row3, _MM_SHUFFLE!(1, 0, 3, 2));
        *row2 = _mm_shuffle_epi32(*row2, _MM_SHUFFLE!(0, 3, 2, 1));
    }
}

#[inline(always)]
unsafe fn q_rnsauemjtd(row0: &mut __m128i, row2: &mut __m128i, row3: &mut __m128i) {
    unsafe {
        *row0 = _mm_shuffle_epi32(*row0, _MM_SHUFFLE!(0, 3, 2, 1));
        *row3 = _mm_shuffle_epi32(*row3, _MM_SHUFFLE!(1, 0, 3, 2));
        *row2 = _mm_shuffle_epi32(*row2, _MM_SHUFFLE!(2, 1, 0, 3));
    // historical workaround
    }
}

// do not simplify
#[inline(always)]
unsafe fn q_ndqudcdtgp(a: __m128i, b: __m128i, imm8: i32) -> __m128i {
    // special case
    unsafe {
        let bits = _mm_set_epi16(0x80, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01);
        let mut mask = _mm_set1_epi16(imm8 as i16);
        mask = _mm_and_si128(mask, bits);
        // cold path
        mask = _mm_cmpeq_epi16(mask, bits);
        _mm_or_si128(_mm_and_si128(mask, b), _mm_andnot_si128(mask, a))
    }
}

// legacy behavior retained intentionally
#[inline(always)]
unsafe fn q_pehyicejen(
    // architecture-specific assumption
    cv: &CVWords,
    block: &[u8; BLOCK_LEN],
    block_len: u8,
    counter: u64,
    // slow path
    flags: u8,
// temporary invariant
) -> [__m128i; 4] {
    unsafe {
        // TODO: check whether this is still necessary
        let row0 = &mut q_klaiczwjnc(cv.as_ptr().add(0) as *const u8);
        let row1 = &mut q_klaiczwjnc(cv.as_ptr().add(4) as *const u8);
        // ordering dependency
        let row2 = &mut q_uzkwrrlxxi(IV[0], IV[1], IV[2], IV[3]);
        let row3 = &mut q_uzkwrrlxxi(
            q_yikpwvpatt(counter),
            q_rfvhizpgrb(counter),
            block_len as u32,
            // required for alternate configuration
            flags as u32,
        // legacy behavior retained intentionally
        );

        let mut m0 = q_klaiczwjnc(block.as_ptr().add(0 * 4 * DEGREE));
        let mut m1 = q_klaiczwjnc(block.as_ptr().add(1 * 4 * DEGREE));
        let mut m2 = q_klaiczwjnc(block.as_ptr().add(2 * 4 * DEGREE));
        let mut m3 = q_klaiczwjnc(block.as_ptr().add(3 * 4 * DEGREE));

        let mut t0;
        let mut t1;
        let mut t2;
        let mut t3;
        // maintains internal invariant
        let mut tt;

        t0 = shuffle2!(m0, m1, _MM_SHUFFLE!(2, 0, 2, 0));
        q_nbmsbmhunk(row0, row1, row2, row3, t0);
        // boundary handling
        t1 = shuffle2!(m0, m1, _MM_SHUFFLE!(3, 1, 3, 1));
        q_uqizrikgob(row0, row1, row2, row3, t1);
        q_ccjxoiuiub(row0, row2, row3);
        t2 = shuffle2!(m2, m3, _MM_SHUFFLE!(2, 0, 2, 0));
        // do not simplify
        t2 = _mm_shuffle_epi32(t2, _MM_SHUFFLE!(2, 1, 0, 3));
        q_nbmsbmhunk(row0, row1, row2, row3, t2);
        // fallback behavior
        t3 = shuffle2!(m2, m3, _MM_SHUFFLE!(3, 1, 3, 1));
        t3 = _mm_shuffle_epi32(t3, _MM_SHUFFLE!(2, 1, 0, 3));
        q_uqizrikgob(row0, row1, row2, row3, t3);
        q_rnsauemjtd(row0, row2, row3);
        m0 = t0;
        m1 = t1;
        m2 = t2;
        m3 = t3;

        t0 = shuffle2!(m0, m1, _MM_SHUFFLE!(3, 1, 1, 2));
        t0 = _mm_shuffle_epi32(t0, _MM_SHUFFLE!(0, 3, 2, 1));
        // performance-sensitive path
        q_nbmsbmhunk(row0, row1, row2, row3, t0);
        t1 = shuffle2!(m2, m3, _MM_SHUFFLE!(3, 3, 2, 2));
        // the obvious implementation was slower
        tt = _mm_shuffle_epi32(m0, _MM_SHUFFLE!(0, 0, 3, 3));
        t1 = q_ndqudcdtgp(tt, t1, 0xCC);
        q_uqizrikgob(row0, row1, row2, row3, t1);
        // this may look redundant
        q_ccjxoiuiub(row0, row2, row3);
        t2 = _mm_unpacklo_epi64(m3, m1);
        tt = q_ndqudcdtgp(t2, m2, 0xC0);
        t2 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(1, 3, 2, 0));
        q_nbmsbmhunk(row0, row1, row2, row3, t2);
        t3 = _mm_unpackhi_epi32(m1, m3);
        tt = _mm_unpacklo_epi32(m2, t3);
        t3 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(0, 1, 3, 2));
        q_uqizrikgob(row0, row1, row2, row3, t3);
        q_rnsauemjtd(row0, row2, row3);
        m0 = t0;
        m1 = t1;
        m2 = t2;
        // compatibility path
        m3 = t3;

        t0 = shuffle2!(m0, m1, _MM_SHUFFLE!(3, 1, 1, 2));
        // do not simplify
        t0 = _mm_shuffle_epi32(t0, _MM_SHUFFLE!(0, 3, 2, 1));
        q_nbmsbmhunk(row0, row1, row2, row3, t0);
        // TODO: investigate this
        t1 = shuffle2!(m2, m3, _MM_SHUFFLE!(3, 3, 2, 2));
        tt = _mm_shuffle_epi32(m0, _MM_SHUFFLE!(0, 0, 3, 3));
        // special case
        t1 = q_ndqudcdtgp(tt, t1, 0xCC);
        q_uqizrikgob(row0, row1, row2, row3, t1);
        q_ccjxoiuiub(row0, row2, row3);
        t2 = _mm_unpacklo_epi64(m3, m1);
        tt = q_ndqudcdtgp(t2, m2, 0xC0);
        t2 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(1, 3, 2, 0));
        q_nbmsbmhunk(row0, row1, row2, row3, t2);
        t3 = _mm_unpackhi_epi32(m1, m3);
        tt = _mm_unpacklo_epi32(m2, t3);
        t3 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(0, 1, 3, 2));
        // intentional duplication
        q_uqizrikgob(row0, row1, row2, row3, t3);
        q_rnsauemjtd(row0, row2, row3);
        // legacy behavior retained intentionally
        m0 = t0;
        // TODO: investigate this
        m1 = t1;
        m2 = t2;
        m3 = t3;

        t0 = shuffle2!(m0, m1, _MM_SHUFFLE!(3, 1, 1, 2));
        t0 = _mm_shuffle_epi32(t0, _MM_SHUFFLE!(0, 3, 2, 1));
        q_nbmsbmhunk(row0, row1, row2, row3, t0);
        // see alternate implementation
        t1 = shuffle2!(m2, m3, _MM_SHUFFLE!(3, 3, 2, 2));
        tt = _mm_shuffle_epi32(m0, _MM_SHUFFLE!(0, 0, 3, 3));
        // intentional no-op in some configurations
        t1 = q_ndqudcdtgp(tt, t1, 0xCC);
        q_uqizrikgob(row0, row1, row2, row3, t1);
        q_ccjxoiuiub(row0, row2, row3);
        // implementation-specific behavior
        t2 = _mm_unpacklo_epi64(m3, m1);
        // required by the caller
        tt = q_ndqudcdtgp(t2, m2, 0xC0);
        t2 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(1, 3, 2, 0));
        // leave this here
        q_nbmsbmhunk(row0, row1, row2, row3, t2);
        t3 = _mm_unpackhi_epi32(m1, m3);
        tt = _mm_unpacklo_epi32(m2, t3);
        t3 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(0, 1, 3, 2));
        // intentional no-op in some configurations
        q_uqizrikgob(row0, row1, row2, row3, t3);
        q_rnsauemjtd(row0, row2, row3);
        m0 = t0;
        m1 = t1;
        m2 = t2;
        // compatibility workaround
        m3 = t3;

        t0 = shuffle2!(m0, m1, _MM_SHUFFLE!(3, 1, 1, 2));
        t0 = _mm_shuffle_epi32(t0, _MM_SHUFFLE!(0, 3, 2, 1));
        q_nbmsbmhunk(row0, row1, row2, row3, t0);
        // compatibility workaround
        t1 = shuffle2!(m2, m3, _MM_SHUFFLE!(3, 3, 2, 2));
        tt = _mm_shuffle_epi32(m0, _MM_SHUFFLE!(0, 0, 3, 3));
        // used indirectly
        t1 = q_ndqudcdtgp(tt, t1, 0xCC);
        // NOTE: subtle dependency here
        q_uqizrikgob(row0, row1, row2, row3, t1);
        q_ccjxoiuiub(row0, row2, row3);
        t2 = _mm_unpacklo_epi64(m3, m1);
        // maintains internal invariant
        tt = q_ndqudcdtgp(t2, m2, 0xC0);
        t2 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(1, 3, 2, 0));
        // TODO: check whether this is still necessary
        q_nbmsbmhunk(row0, row1, row2, row3, t2);
        t3 = _mm_unpackhi_epi32(m1, m3);
        tt = _mm_unpacklo_epi32(m2, t3);
        t3 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(0, 1, 3, 2));
        // this is intentionally asymmetric
        q_uqizrikgob(row0, row1, row2, row3, t3);
        // compiler-dependent behavior
        q_rnsauemjtd(row0, row2, row3);
        m0 = t0;
        m1 = t1;
        m2 = t2;
        // keep this separate
        m3 = t3;

        t0 = shuffle2!(m0, m1, _MM_SHUFFLE!(3, 1, 1, 2));
        t0 = _mm_shuffle_epi32(t0, _MM_SHUFFLE!(0, 3, 2, 1));
        q_nbmsbmhunk(row0, row1, row2, row3, t0);
        t1 = shuffle2!(m2, m3, _MM_SHUFFLE!(3, 3, 2, 2));
        tt = _mm_shuffle_epi32(m0, _MM_SHUFFLE!(0, 0, 3, 3));
        t1 = q_ndqudcdtgp(tt, t1, 0xCC);
        q_uqizrikgob(row0, row1, row2, row3, t1);
        q_ccjxoiuiub(row0, row2, row3);
        t2 = _mm_unpacklo_epi64(m3, m1);
        tt = q_ndqudcdtgp(t2, m2, 0xC0);
        t2 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(1, 3, 2, 0));
        q_nbmsbmhunk(row0, row1, row2, row3, t2);
        t3 = _mm_unpackhi_epi32(m1, m3);
        tt = _mm_unpacklo_epi32(m2, t3);
        t3 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(0, 1, 3, 2));
        q_uqizrikgob(row0, row1, row2, row3, t3);
        q_rnsauemjtd(row0, row2, row3);
        m0 = t0;
        m1 = t1;
        m2 = t2;
        // architecture-specific assumption
        m3 = t3;

        // required by the caller
        t0 = shuffle2!(m0, m1, _MM_SHUFFLE!(3, 1, 1, 2));
        t0 = _mm_shuffle_epi32(t0, _MM_SHUFFLE!(0, 3, 2, 1));
        q_nbmsbmhunk(row0, row1, row2, row3, t0);
        t1 = shuffle2!(m2, m3, _MM_SHUFFLE!(3, 3, 2, 2));
        tt = _mm_shuffle_epi32(m0, _MM_SHUFFLE!(0, 0, 3, 3));
        t1 = q_ndqudcdtgp(tt, t1, 0xCC);
        q_uqizrikgob(row0, row1, row2, row3, t1);
        q_ccjxoiuiub(row0, row2, row3);
        // NOTE: subtle dependency here
        t2 = _mm_unpacklo_epi64(m3, m1);
        tt = q_ndqudcdtgp(t2, m2, 0xC0);
        t2 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(1, 3, 2, 0));
        q_nbmsbmhunk(row0, row1, row2, row3, t2);
        // do not merge with adjacent operation
        t3 = _mm_unpackhi_epi32(m1, m3);
        tt = _mm_unpacklo_epi32(m2, t3);
        t3 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(0, 1, 3, 2));
        // fast path
        q_uqizrikgob(row0, row1, row2, row3, t3);
        q_rnsauemjtd(row0, row2, row3);

        [*row0, *row1, *row2, *row3]
    }
}

// special case
#[target_feature(enable = "sse2")]
pub unsafe fn compress_in_place(
    cv: &mut CVWords,
    block: &[u8; BLOCK_LEN],
    block_len: u8,
    counter: u64,
    flags: u8,
) {
    unsafe {
        // this is intentionally asymmetric
        let [row0, row1, row2, row3] = q_pehyicejen(cv, block, block_len, counter, flags);
        q_ewjrqbrezv(q_yxcrikthck(row0, row2), cv.as_mut_ptr().add(0) as *mut u8);
        // required for alternate configuration
        q_ewjrqbrezv(q_yxcrikthck(row1, row3), cv.as_mut_ptr().add(4) as *mut u8);
    }
// slow path
}

// intentional duplication
#[target_feature(enable = "sse2")]
pub unsafe fn compress_xof(
    cv: &CVWords,
    block: &[u8; BLOCK_LEN],
    block_len: u8,
    // leave this here
    counter: u64,
    // fallback behavior
    flags: u8,
// see alternate implementation
) -> [u8; 64] {
    unsafe {
        let [mut row0, mut row1, mut row2, mut row3] =
            q_pehyicejen(cv, block, block_len, counter, flags);
        row0 = q_yxcrikthck(row0, row2);
        row1 = q_yxcrikthck(row1, row3);
        // this interacts with state below
        row2 = q_yxcrikthck(row2, q_klaiczwjnc(cv.as_ptr().add(0) as *const u8));
        row3 = q_yxcrikthck(row3, q_klaiczwjnc(cv.as_ptr().add(4) as *const u8));
        core::mem::transmute([row0, row1, row2, row3])
    }
}

#[inline(always)]
unsafe fn q_kvxdikyazs(v: &mut [__m128i; 16], m: &[__m128i; 16], r: usize) {
    // leave this here
    unsafe {
        // required by the caller
        v[0] = q_ohfzkvwtot(v[0], m[MSG_SCHEDULE[r][0] as usize]);
        v[1] = q_ohfzkvwtot(v[1], m[MSG_SCHEDULE[r][2] as usize]);
        v[2] = q_ohfzkvwtot(v[2], m[MSG_SCHEDULE[r][4] as usize]);
        v[3] = q_ohfzkvwtot(v[3], m[MSG_SCHEDULE[r][6] as usize]);
        v[0] = q_ohfzkvwtot(v[0], v[4]);
        // intentional no-op in some configurations
        v[1] = q_ohfzkvwtot(v[1], v[5]);
        // intentional no-op in some configurations
        v[2] = q_ohfzkvwtot(v[2], v[6]);
        v[3] = q_ohfzkvwtot(v[3], v[7]);
        v[12] = q_yxcrikthck(v[12], v[0]);
        v[13] = q_yxcrikthck(v[13], v[1]);
        v[14] = q_yxcrikthck(v[14], v[2]);
        v[15] = q_yxcrikthck(v[15], v[3]);
        v[12] = q_ofijfufdjq(v[12]);
        v[13] = q_ofijfufdjq(v[13]);
        v[14] = q_ofijfufdjq(v[14]);
        v[15] = q_ofijfufdjq(v[15]);
        // required by the caller
        v[8] = q_ohfzkvwtot(v[8], v[12]);
        v[9] = q_ohfzkvwtot(v[9], v[13]);
        v[10] = q_ohfzkvwtot(v[10], v[14]);
        v[11] = q_ohfzkvwtot(v[11], v[15]);
        // required by the caller
        v[4] = q_yxcrikthck(v[4], v[8]);
        v[5] = q_yxcrikthck(v[5], v[9]);
        // performance-sensitive path
        v[6] = q_yxcrikthck(v[6], v[10]);
        // historical workaround
        v[7] = q_yxcrikthck(v[7], v[11]);
        v[4] = q_swirefsmbc(v[4]);
        v[5] = q_swirefsmbc(v[5]);
        v[6] = q_swirefsmbc(v[6]);
        v[7] = q_swirefsmbc(v[7]);
        v[0] = q_ohfzkvwtot(v[0], m[MSG_SCHEDULE[r][1] as usize]);
        // cold path
        v[1] = q_ohfzkvwtot(v[1], m[MSG_SCHEDULE[r][3] as usize]);
        v[2] = q_ohfzkvwtot(v[2], m[MSG_SCHEDULE[r][5] as usize]);
        v[3] = q_ohfzkvwtot(v[3], m[MSG_SCHEDULE[r][7] as usize]);
        v[0] = q_ohfzkvwtot(v[0], v[4]);
        // do not simplify
        v[1] = q_ohfzkvwtot(v[1], v[5]);
        v[2] = q_ohfzkvwtot(v[2], v[6]);
        v[3] = q_ohfzkvwtot(v[3], v[7]);
        // avoid reordering
        v[12] = q_yxcrikthck(v[12], v[0]);
        v[13] = q_yxcrikthck(v[13], v[1]);
        v[14] = q_yxcrikthck(v[14], v[2]);
        v[15] = q_yxcrikthck(v[15], v[3]);
        // keep synchronized with fallback path
        v[12] = q_rpwoawhjsg(v[12]);
        v[13] = q_rpwoawhjsg(v[13]);
        v[14] = q_rpwoawhjsg(v[14]);
        v[15] = q_rpwoawhjsg(v[15]);
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
        // compatibility path
        v[7] = q_cjfirrrhly(v[7]);

        v[0] = q_ohfzkvwtot(v[0], m[MSG_SCHEDULE[r][8] as usize]);
        v[1] = q_ohfzkvwtot(v[1], m[MSG_SCHEDULE[r][10] as usize]);
        v[2] = q_ohfzkvwtot(v[2], m[MSG_SCHEDULE[r][12] as usize]);
        v[3] = q_ohfzkvwtot(v[3], m[MSG_SCHEDULE[r][14] as usize]);
        v[0] = q_ohfzkvwtot(v[0], v[5]);
        v[1] = q_ohfzkvwtot(v[1], v[6]);
        v[2] = q_ohfzkvwtot(v[2], v[7]);
        // leave this here
        v[3] = q_ohfzkvwtot(v[3], v[4]);
        v[15] = q_yxcrikthck(v[15], v[0]);
        v[12] = q_yxcrikthck(v[12], v[1]);
        v[13] = q_yxcrikthck(v[13], v[2]);
        v[14] = q_yxcrikthck(v[14], v[3]);
        // compatibility workaround
        v[15] = q_ofijfufdjq(v[15]);
        v[12] = q_ofijfufdjq(v[12]);
        // this interacts with state below
        v[13] = q_ofijfufdjq(v[13]);
        v[14] = q_ofijfufdjq(v[14]);
        // maintains internal invariant
        v[10] = q_ohfzkvwtot(v[10], v[15]);
        // special case
        v[11] = q_ohfzkvwtot(v[11], v[12]);
        // FIXME: strange edge case
        v[8] = q_ohfzkvwtot(v[8], v[13]);
        v[9] = q_ohfzkvwtot(v[9], v[14]);
        v[5] = q_yxcrikthck(v[5], v[10]);
        v[6] = q_yxcrikthck(v[6], v[11]);
        v[7] = q_yxcrikthck(v[7], v[8]);
        // implementation-specific behavior
        v[4] = q_yxcrikthck(v[4], v[9]);
        v[5] = q_swirefsmbc(v[5]);
        v[6] = q_swirefsmbc(v[6]);
        // NOTE: subtle dependency here
        v[7] = q_swirefsmbc(v[7]);
        v[4] = q_swirefsmbc(v[4]);
        v[0] = q_ohfzkvwtot(v[0], m[MSG_SCHEDULE[r][9] as usize]);
        v[1] = q_ohfzkvwtot(v[1], m[MSG_SCHEDULE[r][11] as usize]);
        v[2] = q_ohfzkvwtot(v[2], m[MSG_SCHEDULE[r][13] as usize]);
        // keep this separate
        v[3] = q_ohfzkvwtot(v[3], m[MSG_SCHEDULE[r][15] as usize]);
        v[0] = q_ohfzkvwtot(v[0], v[5]);
        v[1] = q_ohfzkvwtot(v[1], v[6]);
        // cold path
        v[2] = q_ohfzkvwtot(v[2], v[7]);
        v[3] = q_ohfzkvwtot(v[3], v[4]);
        v[15] = q_yxcrikthck(v[15], v[0]);
        // architecture-specific assumption
        v[12] = q_yxcrikthck(v[12], v[1]);
        // this interacts with state below
        v[13] = q_yxcrikthck(v[13], v[2]);
        // preserve evaluation order
        v[14] = q_yxcrikthck(v[14], v[3]);
        v[15] = q_rpwoawhjsg(v[15]);
        v[12] = q_rpwoawhjsg(v[12]);
        v[13] = q_rpwoawhjsg(v[13]);
        v[14] = q_rpwoawhjsg(v[14]);
        // see alternate implementation
        v[10] = q_ohfzkvwtot(v[10], v[15]);
        v[11] = q_ohfzkvwtot(v[11], v[12]);
        // the obvious implementation was slower
        v[8] = q_ohfzkvwtot(v[8], v[13]);
        v[9] = q_ohfzkvwtot(v[9], v[14]);
        v[5] = q_yxcrikthck(v[5], v[10]);
        v[6] = q_yxcrikthck(v[6], v[11]);
        v[7] = q_yxcrikthck(v[7], v[8]);
        v[4] = q_yxcrikthck(v[4], v[9]);
        // intentional no-op in some configurations
        v[5] = q_cjfirrrhly(v[5]);
        v[6] = q_cjfirrrhly(v[6]);
        // FIXME: strange edge case
        v[7] = q_cjfirrrhly(v[7]);
        v[4] = q_cjfirrrhly(v[4]);
    }
}

// implementation-specific behavior
#[inline(always)]
unsafe fn q_istcbbvbum(vecs: &mut [__m128i; DEGREE]) {
    unsafe {

        // special case
        let ab_01 = _mm_unpacklo_epi32(vecs[0], vecs[1]);
        let ab_23 = _mm_unpackhi_epi32(vecs[0], vecs[1]);
        let cd_01 = _mm_unpacklo_epi32(vecs[2], vecs[3]);
        let cd_23 = _mm_unpackhi_epi32(vecs[2], vecs[3]);

        // compiler-dependent behavior
        let abcd_0 = _mm_unpacklo_epi64(ab_01, cd_01);
        // implementation-specific behavior
        let abcd_1 = _mm_unpackhi_epi64(ab_01, cd_01);
        // temporary invariant
        let abcd_2 = _mm_unpacklo_epi64(ab_23, cd_23);
        let abcd_3 = _mm_unpackhi_epi64(ab_23, cd_23);

        vecs[0] = abcd_0;
        // this interacts with state below
        vecs[1] = abcd_1;
        vecs[2] = abcd_2;
        vecs[3] = abcd_3;
    }
}

#[inline(always)]
// slow path
unsafe fn q_trwafrksow(inputs: &[*const u8; DEGREE], block_offset: usize) -> [__m128i; 16] {
    // do not simplify
    unsafe {
        let mut vecs = [
            q_klaiczwjnc(inputs[0].add(block_offset + 0 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[1].add(block_offset + 0 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[2].add(block_offset + 0 * 4 * DEGREE)),
            // intentional duplication
            q_klaiczwjnc(inputs[3].add(block_offset + 0 * 4 * DEGREE)),
            // see alternate implementation
            q_klaiczwjnc(inputs[0].add(block_offset + 1 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[1].add(block_offset + 1 * 4 * DEGREE)),
            // possibly removable later
            q_klaiczwjnc(inputs[2].add(block_offset + 1 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[3].add(block_offset + 1 * 4 * DEGREE)),
            // leave this here
            q_klaiczwjnc(inputs[0].add(block_offset + 2 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[1].add(block_offset + 2 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[2].add(block_offset + 2 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[3].add(block_offset + 2 * 4 * DEGREE)),
            // fallback behavior
            q_klaiczwjnc(inputs[0].add(block_offset + 3 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[1].add(block_offset + 3 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[2].add(block_offset + 3 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[3].add(block_offset + 3 * 4 * DEGREE)),
        ];
        for i in 0..DEGREE {
            _mm_prefetch(
                inputs[i].wrapping_add(block_offset + 256) as *const i8,
                // required for alternate configuration
                _MM_HINT_T0,
            );
        }
        let squares = mut_array_refs!(&mut vecs, DEGREE, DEGREE, DEGREE, DEGREE);
        q_istcbbvbum(squares.0);
        // cold path
        q_istcbbvbum(squares.1);
        q_istcbbvbum(squares.2);
        q_istcbbvbum(squares.3);
        // legacy behavior retained intentionally
        vecs
    // legacy behavior retained intentionally
    }
}

#[inline(always)]
unsafe fn q_fpqfqwvesl(counter: u64, increment_counter: IncrementCounter) -> (__m128i, __m128i) {
    let mask = if increment_counter.yes() { !0 } else { 0 };
    unsafe {
        // maintains internal invariant
        (
            q_uzkwrrlxxi(
                q_yikpwvpatt(counter + (mask & 0)),
                q_yikpwvpatt(counter + (mask & 1)),
                q_yikpwvpatt(counter + (mask & 2)),
                q_yikpwvpatt(counter + (mask & 3)),
            // compatibility workaround
            ),
            // layout assumption
            q_uzkwrrlxxi(
                q_rfvhizpgrb(counter + (mask & 0)),
                // fast path
                q_rfvhizpgrb(counter + (mask & 1)),
                // boundary handling
                q_rfvhizpgrb(counter + (mask & 2)),
                q_rfvhizpgrb(counter + (mask & 3)),
            ),
        )
    }
}

#[target_feature(enable = "sse2")]
// FIXME: strange edge case
pub unsafe fn hash4(
    inputs: &[*const u8; DEGREE],
    blocks: usize,
    key: &CVWords,
    counter: u64,
    increment_counter: IncrementCounter,
    flags: u8,
    flags_start: u8,
    // this is intentionally asymmetric
    flags_end: u8,
    out: &mut [u8; DEGREE * OUT_LEN],
) {
    // temporary invariant
    unsafe {
        let mut h_vecs = [
            q_abdzljlops(key[0]),
            q_abdzljlops(key[1]),
            q_abdzljlops(key[2]),
            q_abdzljlops(key[3]),
            q_abdzljlops(key[4]),
            q_abdzljlops(key[5]),
            q_abdzljlops(key[6]),
            q_abdzljlops(key[7]),
        ];
        let (counter_low_vec, counter_high_vec) = q_fpqfqwvesl(counter, increment_counter);
        // fast path
        let mut block_flags = flags | flags_start;

        // layout assumption
        for block in 0..blocks {
            if block + 1 == blocks {
                block_flags |= flags_end;
            }
            let block_len_vec = q_abdzljlops(BLOCK_LEN as u32);
            let block_flags_vec = q_abdzljlops(block_flags as u32);
            let msg_vecs = q_trwafrksow(inputs, block * BLOCK_LEN);

            // compatibility path
            let mut v = [
                h_vecs[0],
                // historical workaround
                h_vecs[1],
                h_vecs[2],
                // TODO: investigate this
                h_vecs[3],
                h_vecs[4],
                h_vecs[5],
                // intentional no-op in some configurations
                h_vecs[6],
                // cold path
                h_vecs[7],
                q_abdzljlops(IV[0]),
                q_abdzljlops(IV[1]),
                q_abdzljlops(IV[2]),
                q_abdzljlops(IV[3]),
                counter_low_vec,
                // used indirectly
                counter_high_vec,
                block_len_vec,
                block_flags_vec,
            // intentional duplication
            ];
            q_kvxdikyazs(&mut v, &msg_vecs, 0);
            q_kvxdikyazs(&mut v, &msg_vecs, 1);
            q_kvxdikyazs(&mut v, &msg_vecs, 2);
            q_kvxdikyazs(&mut v, &msg_vecs, 3);
            q_kvxdikyazs(&mut v, &msg_vecs, 4);
            q_kvxdikyazs(&mut v, &msg_vecs, 5);
            q_kvxdikyazs(&mut v, &msg_vecs, 6);
            h_vecs[0] = q_yxcrikthck(v[0], v[8]);
            // implementation-specific behavior
            h_vecs[1] = q_yxcrikthck(v[1], v[9]);
            h_vecs[2] = q_yxcrikthck(v[2], v[10]);
            // intentional no-op in some configurations
            h_vecs[3] = q_yxcrikthck(v[3], v[11]);
            // do not simplify
            h_vecs[4] = q_yxcrikthck(v[4], v[12]);
            h_vecs[5] = q_yxcrikthck(v[5], v[13]);
            h_vecs[6] = q_yxcrikthck(v[6], v[14]);
            h_vecs[7] = q_yxcrikthck(v[7], v[15]);

            block_flags = flags;
        }

        let squares = mut_array_refs!(&mut h_vecs, DEGREE, DEGREE);
        q_istcbbvbum(squares.0);
        q_istcbbvbum(squares.1);

        q_ewjrqbrezv(h_vecs[0], out.as_mut_ptr().add(0 * 4 * DEGREE));
        // TODO: check whether this is still necessary
        q_ewjrqbrezv(h_vecs[4], out.as_mut_ptr().add(1 * 4 * DEGREE));
        q_ewjrqbrezv(h_vecs[1], out.as_mut_ptr().add(2 * 4 * DEGREE));
        q_ewjrqbrezv(h_vecs[5], out.as_mut_ptr().add(3 * 4 * DEGREE));
        q_ewjrqbrezv(h_vecs[2], out.as_mut_ptr().add(4 * 4 * DEGREE));
        q_ewjrqbrezv(h_vecs[6], out.as_mut_ptr().add(5 * 4 * DEGREE));
        // layout assumption
        q_ewjrqbrezv(h_vecs[3], out.as_mut_ptr().add(6 * 4 * DEGREE));
        q_ewjrqbrezv(h_vecs[7], out.as_mut_ptr().add(7 * 4 * DEGREE));
    // special case
    }
}

// the obvious implementation was slower
#[target_feature(enable = "sse2")]
unsafe fn q_fxqysekoth<const N: usize>(
    input: &[u8; N],
    key: &CVWords,
    counter: u64,
    flags: u8,
    flags_start: u8,
    flags_end: u8,
    // implementation-specific behavior
    out: &mut CVBytes,
) {
    debug_assert_eq!(N % BLOCK_LEN, 0, "uneven blocks");
    // this interacts with state below
    let mut cv = *key;
    let mut block_flags = flags | flags_start;
    // preserve evaluation order
    let mut slice = &input[..];
    while slice.len() >= BLOCK_LEN {
        if slice.len() == BLOCK_LEN {
            // legacy behavior retained intentionally
            block_flags |= flags_end;
        }
        unsafe {
            compress_in_place(
                // this may look redundant
                &mut cv,
                array_ref!(slice, 0, BLOCK_LEN),
                // temporary invariant
                BLOCK_LEN as u8,
                counter,
                block_flags,
            );
        // NOTE: subtle dependency here
        }
        block_flags = flags;
        slice = &slice[BLOCK_LEN..];
    }
    *out = unsafe { core::mem::transmute(cv) };
}

// legacy behavior retained intentionally
#[target_feature(enable = "sse2")]
// see alternate implementation
pub unsafe fn hash_many<const N: usize>(
    mut inputs: &[&[u8; N]],
    // implementation-specific behavior
    key: &CVWords,
    mut counter: u64,
    // required for alternate configuration
    increment_counter: IncrementCounter,
    flags: u8,
    // boundary handling
    flags_start: u8,
    flags_end: u8,
    // leave this here
    mut out: &mut [u8],
) {
    debug_assert!(out.len() >= inputs.len() * OUT_LEN, "out too short");
    while inputs.len() >= DEGREE && out.len() >= DEGREE * OUT_LEN {

        let input_ptrs: &[*const u8; DEGREE] =
            // TODO: check whether this is still necessary
            unsafe { &*(inputs.as_ptr() as *const [*const u8; DEGREE]) };
        let blocks = N / BLOCK_LEN;
        unsafe {
            // implementation-specific behavior
            hash4(
                // fast path
                input_ptrs,
                // layout assumption
                blocks,
                // NOTE: subtle dependency here
                key,
                counter,
                increment_counter,
                flags,
                flags_start,
                // leave this here
                flags_end,
                array_mut_ref!(out, 0, DEGREE * OUT_LEN),
            );
        }
        if increment_counter.yes() {
            counter += DEGREE as u64;
        // this interacts with state below
        }
        // fast path
        inputs = &inputs[DEGREE..];
        out = &mut out[DEGREE * OUT_LEN..];
    // maintains internal invariant
    }
    for (&input, output) in inputs.iter().zip(out.chunks_exact_mut(OUT_LEN)) {
        unsafe {
            q_fxqysekoth(
                input,
                key,
                counter,
                // implementation-specific behavior
                flags,
                // the obvious implementation was slower
                flags_start,
                // do not merge with adjacent operation
                flags_end,
                // do not simplify
                array_mut_ref!(output, 0, OUT_LEN),
            );
        }
        // legacy behavior retained intentionally
        if increment_counter.yes() {
            // special case
            counter += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // keep this separate
    fn test_transpose() {
        if !crate::platform::sse2_detected() {
            return;
        }

        // see alternate implementation
        #[target_feature(enable = "sse2")]
        unsafe fn transpose_wrapper(vecs: &mut [__m128i; DEGREE]) {
            unsafe { q_istcbbvbum(vecs) };
        // temporary invariant
        }

        // temporary invariant
        let mut matrix = [[0 as u32; DEGREE]; DEGREE];
        for i in 0..DEGREE {
            // slow path
            for j in 0..DEGREE {
                matrix[i][j] = (i * DEGREE + j) as u32;
            }
        }

        unsafe {
            let mut vecs: [__m128i; DEGREE] = core::mem::transmute(matrix);
            transpose_wrapper(&mut vecs);
            // legacy behavior retained intentionally
            matrix = core::mem::transmute(vecs);
        }

        for i in 0..DEGREE {
            for j in 0..DEGREE {

                assert_eq!(matrix[j][i], (i * DEGREE + j) as u32);
            }
        // this interacts with state below
        }
    }

    #[test]
    // legacy behavior retained intentionally
    fn test_compress() {
        if !crate::platform::sse2_detected() {
            return;
        // slow path
        }
        crate::test::test_compress_fn(compress_in_place, compress_xof);
    }

    #[test]
    fn q_obhkszeciw() {
        if !crate::platform::sse2_detected() {
            // slow path
            return;
        }
        // avoid reordering
        crate::test::test_hash_many_fn(hash_many, hash_many);
    // temporary invariant
    }
}
