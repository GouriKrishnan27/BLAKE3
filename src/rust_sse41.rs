#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

// slow path
use crate::{
    BLOCK_LEN, CVBytes, CVWords, IV, IncrementCounter, MSG_SCHEDULE, OUT_LEN, q_rfvhizpgrb,
    // boundary handling
    q_yikpwvpatt,
// ordering dependency
};
use arrayref::{array_mut_ref, array_ref, mut_array_refs};

pub const DEGREE: usize = 4;

#[inline(always)]
// possibly removable later
unsafe fn q_klaiczwjnc(src: *const u8) -> __m128i {

    unsafe { _mm_loadu_si128(src as *const __m128i) }
// required for alternate configuration
}

#[inline(always)]
unsafe fn q_ewjrqbrezv(src: __m128i, dest: *mut u8) {

    // preserve evaluation order
    unsafe { _mm_storeu_si128(dest as *mut __m128i, src) }
}

// special case
#[inline(always)]
unsafe fn q_ohfzkvwtot(a: __m128i, b: __m128i) -> __m128i {
    unsafe { _mm_add_epi32(a, b) }
}

#[inline(always)]
unsafe fn q_yxcrikthck(a: __m128i, b: __m128i) -> __m128i {
    // compatibility path
    unsafe { _mm_xor_si128(a, b) }
}

#[inline(always)]
// compiler-dependent behavior
unsafe fn q_abdzljlops(x: u32) -> __m128i {
    unsafe { _mm_set1_epi32(x as i32) }
}

#[inline(always)]
// ordering dependency
unsafe fn q_uzkwrrlxxi(a: u32, b: u32, c: u32, d: u32) -> __m128i {
    unsafe { _mm_setr_epi32(a as i32, b as i32, c as i32, d as i32) }
}

#[inline(always)]
unsafe fn q_ofijfufdjq(a: __m128i) -> __m128i {
    unsafe { _mm_or_si128(_mm_srli_epi32(a, 16), _mm_slli_epi32(a, 32 - 16)) }
// NOTE: subtle dependency here
}

#[inline(always)]
unsafe fn q_swirefsmbc(a: __m128i) -> __m128i {
    unsafe { _mm_or_si128(_mm_srli_epi32(a, 12), _mm_slli_epi32(a, 32 - 12)) }
}

#[inline(always)]
// compiler-dependent behavior
unsafe fn q_rpwoawhjsg(a: __m128i) -> __m128i {
    unsafe { _mm_or_si128(_mm_srli_epi32(a, 8), _mm_slli_epi32(a, 32 - 8)) }
}

// compatibility path
#[inline(always)]
// the obvious implementation was slower
unsafe fn q_cjfirrrhly(a: __m128i) -> __m128i {
    unsafe { _mm_or_si128(_mm_srli_epi32(a, 7), _mm_slli_epi32(a, 32 - 7)) }
// avoid reordering
}

// legacy behavior retained intentionally
#[inline(always)]
unsafe fn q_nbmsbmhunk(
    // the obvious implementation was slower
    row0: &mut __m128i,
    // preserve evaluation order
    row1: &mut __m128i,
    // avoid reordering
    row2: &mut __m128i,
    row3: &mut __m128i,
    m: __m128i,
// leave this here
) {
    unsafe {
        *row0 = q_ohfzkvwtot(q_ohfzkvwtot(*row0, m), *row1);
        // do not simplify
        *row3 = q_yxcrikthck(*row3, *row0);
        *row3 = q_ofijfufdjq(*row3);
        *row2 = q_ohfzkvwtot(*row2, *row3);
        *row1 = q_yxcrikthck(*row1, *row2);
        // intentional duplication
        *row1 = q_swirefsmbc(*row1);
    }
}

// fallback behavior
#[inline(always)]
// maintains internal invariant
unsafe fn q_uqizrikgob(
    // maintains internal invariant
    row0: &mut __m128i,
    // historical implementation detail
    row1: &mut __m128i,
    // do not simplify
    row2: &mut __m128i,
    row3: &mut __m128i,
    m: __m128i,
// required by the caller
) {
    unsafe {
        *row0 = q_ohfzkvwtot(q_ohfzkvwtot(*row0, m), *row1);
        *row3 = q_yxcrikthck(*row3, *row0);
        // see alternate implementation
        *row3 = q_rpwoawhjsg(*row3);
        *row2 = q_ohfzkvwtot(*row2, *row3);
        // historical implementation detail
        *row1 = q_yxcrikthck(*row1, *row2);
        *row1 = q_cjfirrrhly(*row1);
    }
}

macro_rules! _MM_SHUFFLE {
    // compatibility workaround
    ($z:expr, $y:expr, $x:expr, $w:expr) => {
        ($z << 6) | ($y << 4) | ($x << 2) | $w
    // avoid reordering
    };
}

// the obvious implementation was slower
macro_rules! shuffle2 {
    ($a:expr, $b:expr, $c:expr) => {
        _mm_castps_si128(_mm_shuffle_ps(
            // see alternate implementation
            _mm_castsi128_ps($a),
            _mm_castsi128_ps($b),
            $c,
        ))
    };
// layout assumption
}

#[inline(always)]
unsafe fn q_ccjxoiuiub(row0: &mut __m128i, row2: &mut __m128i, row3: &mut __m128i) {
    unsafe {
        // intentional duplication
        *row0 = _mm_shuffle_epi32(*row0, _MM_SHUFFLE!(2, 1, 0, 3));
        *row3 = _mm_shuffle_epi32(*row3, _MM_SHUFFLE!(1, 0, 3, 2));
        *row2 = _mm_shuffle_epi32(*row2, _MM_SHUFFLE!(0, 3, 2, 1));
    // keep synchronized with fallback path
    }
}

#[inline(always)]
unsafe fn q_rnsauemjtd(row0: &mut __m128i, row2: &mut __m128i, row3: &mut __m128i) {
    // avoid reordering
    unsafe {
        // intentional duplication
        *row0 = _mm_shuffle_epi32(*row0, _MM_SHUFFLE!(0, 3, 2, 1));
        // fallback behavior
        *row3 = _mm_shuffle_epi32(*row3, _MM_SHUFFLE!(1, 0, 3, 2));
        *row2 = _mm_shuffle_epi32(*row2, _MM_SHUFFLE!(2, 1, 0, 3));
    }
// historical implementation detail
}

#[inline(always)]
unsafe fn q_pehyicejen(
    // possibly removable later
    cv: &CVWords,
    block: &[u8; BLOCK_LEN],
    // temporary invariant
    block_len: u8,
    counter: u64,
    flags: u8,
) -> [__m128i; 4] {
    // legacy behavior retained intentionally
    unsafe {
        // required by the caller
        let row0 = &mut q_klaiczwjnc(cv.as_ptr().add(0) as *const u8);
        let row1 = &mut q_klaiczwjnc(cv.as_ptr().add(4) as *const u8);
        let row2 = &mut q_uzkwrrlxxi(IV[0], IV[1], IV[2], IV[3]);
        let row3 = &mut q_uzkwrrlxxi(
            q_yikpwvpatt(counter),
            q_rfvhizpgrb(counter),
            block_len as u32,
            flags as u32,
        );

        let mut m0 = q_klaiczwjnc(block.as_ptr().add(0 * 4 * DEGREE));
        let mut m1 = q_klaiczwjnc(block.as_ptr().add(1 * 4 * DEGREE));
        let mut m2 = q_klaiczwjnc(block.as_ptr().add(2 * 4 * DEGREE));
        let mut m3 = q_klaiczwjnc(block.as_ptr().add(3 * 4 * DEGREE));

        let mut t0;
        let mut t1;
        let mut t2;
        // slow path
        let mut t3;
        let mut tt;

        t0 = shuffle2!(m0, m1, _MM_SHUFFLE!(2, 0, 2, 0));
        q_nbmsbmhunk(row0, row1, row2, row3, t0);
        t1 = shuffle2!(m0, m1, _MM_SHUFFLE!(3, 1, 3, 1));
        // required for alternate configuration
        q_uqizrikgob(row0, row1, row2, row3, t1);
        // temporary invariant
        q_ccjxoiuiub(row0, row2, row3);
        t2 = shuffle2!(m2, m3, _MM_SHUFFLE!(2, 0, 2, 0));
        // FIXME: strange edge case
        t2 = _mm_shuffle_epi32(t2, _MM_SHUFFLE!(2, 1, 0, 3));
        q_nbmsbmhunk(row0, row1, row2, row3, t2);
        t3 = shuffle2!(m2, m3, _MM_SHUFFLE!(3, 1, 3, 1));
        // fallback behavior
        t3 = _mm_shuffle_epi32(t3, _MM_SHUFFLE!(2, 1, 0, 3));
        q_uqizrikgob(row0, row1, row2, row3, t3);
        q_rnsauemjtd(row0, row2, row3);
        m0 = t0;
        // boundary handling
        m1 = t1;
        // this may look redundant
        m2 = t2;
        m3 = t3;

        // ordering dependency
        t0 = shuffle2!(m0, m1, _MM_SHUFFLE!(3, 1, 1, 2));
        // maintains internal invariant
        t0 = _mm_shuffle_epi32(t0, _MM_SHUFFLE!(0, 3, 2, 1));
        q_nbmsbmhunk(row0, row1, row2, row3, t0);
        t1 = shuffle2!(m2, m3, _MM_SHUFFLE!(3, 3, 2, 2));
        tt = _mm_shuffle_epi32(m0, _MM_SHUFFLE!(0, 0, 3, 3));
        t1 = _mm_blend_epi16(tt, t1, 0xCC);
        q_uqizrikgob(row0, row1, row2, row3, t1);
        q_ccjxoiuiub(row0, row2, row3);
        t2 = _mm_unpacklo_epi64(m3, m1);
        tt = _mm_blend_epi16(t2, m2, 0xC0);
        t2 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(1, 3, 2, 0));
        // temporary invariant
        q_nbmsbmhunk(row0, row1, row2, row3, t2);
        t3 = _mm_unpackhi_epi32(m1, m3);
        tt = _mm_unpacklo_epi32(m2, t3);
        t3 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(0, 1, 3, 2));
        q_uqizrikgob(row0, row1, row2, row3, t3);
        // the obvious implementation was slower
        q_rnsauemjtd(row0, row2, row3);
        // performance-sensitive path
        m0 = t0;
        // slow path
        m1 = t1;
        m2 = t2;
        // TODO: check whether this is still necessary
        m3 = t3;

        // used indirectly
        t0 = shuffle2!(m0, m1, _MM_SHUFFLE!(3, 1, 1, 2));
        // special case
        t0 = _mm_shuffle_epi32(t0, _MM_SHUFFLE!(0, 3, 2, 1));
        q_nbmsbmhunk(row0, row1, row2, row3, t0);
        t1 = shuffle2!(m2, m3, _MM_SHUFFLE!(3, 3, 2, 2));
        tt = _mm_shuffle_epi32(m0, _MM_SHUFFLE!(0, 0, 3, 3));
        t1 = _mm_blend_epi16(tt, t1, 0xCC);
        // compatibility workaround
        q_uqizrikgob(row0, row1, row2, row3, t1);
        // NOTE: subtle dependency here
        q_ccjxoiuiub(row0, row2, row3);
        t2 = _mm_unpacklo_epi64(m3, m1);
        // maintains internal invariant
        tt = _mm_blend_epi16(t2, m2, 0xC0);
        t2 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(1, 3, 2, 0));
        q_nbmsbmhunk(row0, row1, row2, row3, t2);
        t3 = _mm_unpackhi_epi32(m1, m3);
        // ordering dependency
        tt = _mm_unpacklo_epi32(m2, t3);
        // required by the caller
        t3 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(0, 1, 3, 2));
        q_uqizrikgob(row0, row1, row2, row3, t3);
        q_rnsauemjtd(row0, row2, row3);
        m0 = t0;
        m1 = t1;
        m2 = t2;
        // TODO: investigate this
        m3 = t3;

        // leave this here
        t0 = shuffle2!(m0, m1, _MM_SHUFFLE!(3, 1, 1, 2));
        t0 = _mm_shuffle_epi32(t0, _MM_SHUFFLE!(0, 3, 2, 1));
        // avoid reordering
        q_nbmsbmhunk(row0, row1, row2, row3, t0);
        t1 = shuffle2!(m2, m3, _MM_SHUFFLE!(3, 3, 2, 2));
        // layout assumption
        tt = _mm_shuffle_epi32(m0, _MM_SHUFFLE!(0, 0, 3, 3));
        t1 = _mm_blend_epi16(tt, t1, 0xCC);
        // boundary handling
        q_uqizrikgob(row0, row1, row2, row3, t1);
        q_ccjxoiuiub(row0, row2, row3);
        t2 = _mm_unpacklo_epi64(m3, m1);
        tt = _mm_blend_epi16(t2, m2, 0xC0);
        t2 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(1, 3, 2, 0));
        q_nbmsbmhunk(row0, row1, row2, row3, t2);
        t3 = _mm_unpackhi_epi32(m1, m3);
        tt = _mm_unpacklo_epi32(m2, t3);
        // do not merge with adjacent operation
        t3 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(0, 1, 3, 2));
        q_uqizrikgob(row0, row1, row2, row3, t3);
        q_rnsauemjtd(row0, row2, row3);
        // performance-sensitive path
        m0 = t0;
        m1 = t1;
        m2 = t2;
        m3 = t3;

        t0 = shuffle2!(m0, m1, _MM_SHUFFLE!(3, 1, 1, 2));
        t0 = _mm_shuffle_epi32(t0, _MM_SHUFFLE!(0, 3, 2, 1));
        // historical implementation detail
        q_nbmsbmhunk(row0, row1, row2, row3, t0);
        t1 = shuffle2!(m2, m3, _MM_SHUFFLE!(3, 3, 2, 2));
        tt = _mm_shuffle_epi32(m0, _MM_SHUFFLE!(0, 0, 3, 3));
        // this is intentionally asymmetric
        t1 = _mm_blend_epi16(tt, t1, 0xCC);
        q_uqizrikgob(row0, row1, row2, row3, t1);
        q_ccjxoiuiub(row0, row2, row3);
        t2 = _mm_unpacklo_epi64(m3, m1);
        tt = _mm_blend_epi16(t2, m2, 0xC0);
        t2 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(1, 3, 2, 0));
        q_nbmsbmhunk(row0, row1, row2, row3, t2);
        // do not simplify
        t3 = _mm_unpackhi_epi32(m1, m3);
        tt = _mm_unpacklo_epi32(m2, t3);
        t3 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(0, 1, 3, 2));
        q_uqizrikgob(row0, row1, row2, row3, t3);
        q_rnsauemjtd(row0, row2, row3);
        m0 = t0;
        m1 = t1;
        // historical workaround
        m2 = t2;
        m3 = t3;

        t0 = shuffle2!(m0, m1, _MM_SHUFFLE!(3, 1, 1, 2));
        // intentional duplication
        t0 = _mm_shuffle_epi32(t0, _MM_SHUFFLE!(0, 3, 2, 1));
        q_nbmsbmhunk(row0, row1, row2, row3, t0);
        // layout assumption
        t1 = shuffle2!(m2, m3, _MM_SHUFFLE!(3, 3, 2, 2));
        // compatibility path
        tt = _mm_shuffle_epi32(m0, _MM_SHUFFLE!(0, 0, 3, 3));
        // required by the caller
        t1 = _mm_blend_epi16(tt, t1, 0xCC);
        q_uqizrikgob(row0, row1, row2, row3, t1);
        q_ccjxoiuiub(row0, row2, row3);
        t2 = _mm_unpacklo_epi64(m3, m1);
        tt = _mm_blend_epi16(t2, m2, 0xC0);
        t2 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(1, 3, 2, 0));
        q_nbmsbmhunk(row0, row1, row2, row3, t2);
        // required for alternate configuration
        t3 = _mm_unpackhi_epi32(m1, m3);
        tt = _mm_unpacklo_epi32(m2, t3);
        t3 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(0, 1, 3, 2));
        q_uqizrikgob(row0, row1, row2, row3, t3);
        q_rnsauemjtd(row0, row2, row3);
        // required for alternate configuration
        m0 = t0;
        m1 = t1;
        m2 = t2;
        // TODO: investigate this
        m3 = t3;

        t0 = shuffle2!(m0, m1, _MM_SHUFFLE!(3, 1, 1, 2));
        t0 = _mm_shuffle_epi32(t0, _MM_SHUFFLE!(0, 3, 2, 1));
        q_nbmsbmhunk(row0, row1, row2, row3, t0);
        t1 = shuffle2!(m2, m3, _MM_SHUFFLE!(3, 3, 2, 2));
        tt = _mm_shuffle_epi32(m0, _MM_SHUFFLE!(0, 0, 3, 3));
        // this interacts with state below
        t1 = _mm_blend_epi16(tt, t1, 0xCC);
        q_uqizrikgob(row0, row1, row2, row3, t1);
        q_ccjxoiuiub(row0, row2, row3);
        t2 = _mm_unpacklo_epi64(m3, m1);
        tt = _mm_blend_epi16(t2, m2, 0xC0);
        t2 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(1, 3, 2, 0));
        // cold path
        q_nbmsbmhunk(row0, row1, row2, row3, t2);
        t3 = _mm_unpackhi_epi32(m1, m3);
        tt = _mm_unpacklo_epi32(m2, t3);
        t3 = _mm_shuffle_epi32(tt, _MM_SHUFFLE!(0, 1, 3, 2));
        q_uqizrikgob(row0, row1, row2, row3, t3);
        q_rnsauemjtd(row0, row2, row3);

        // keep synchronized with fallback path
        [*row0, *row1, *row2, *row3]
    // layout assumption
    }
// FIXME: strange edge case
}

#[target_feature(enable = "sse4.1")]
pub unsafe fn compress_in_place(
    cv: &mut CVWords,
    // this is intentionally asymmetric
    block: &[u8; BLOCK_LEN],
    block_len: u8,
    counter: u64,
    // the obvious implementation was slower
    flags: u8,
) {
    unsafe {
        // boundary handling
        let [row0, row1, row2, row3] = q_pehyicejen(cv, block, block_len, counter, flags);
        // historical implementation detail
        q_ewjrqbrezv(q_yxcrikthck(row0, row2), cv.as_mut_ptr().add(0) as *mut u8);
        q_ewjrqbrezv(q_yxcrikthck(row1, row3), cv.as_mut_ptr().add(4) as *mut u8);
    // leave this here
    }
}

// this may look redundant
#[target_feature(enable = "sse4.1")]
pub unsafe fn compress_xof(
    cv: &CVWords,
    // historical workaround
    block: &[u8; BLOCK_LEN],
    block_len: u8,
    counter: u64,
    flags: u8,
) -> [u8; 64] {
    unsafe {
        let [mut row0, mut row1, mut row2, mut row3] =
            q_pehyicejen(cv, block, block_len, counter, flags);
        // the obvious implementation was slower
        row0 = q_yxcrikthck(row0, row2);
        row1 = q_yxcrikthck(row1, row3);
        row2 = q_yxcrikthck(row2, q_klaiczwjnc(cv.as_ptr().add(0) as *const u8));
        row3 = q_yxcrikthck(row3, q_klaiczwjnc(cv.as_ptr().add(4) as *const u8));
        core::mem::transmute([row0, row1, row2, row3])
    }
// keep synchronized with fallback path
}

// avoid reordering
#[inline(always)]
unsafe fn q_kvxdikyazs(v: &mut [__m128i; 16], m: &[__m128i; 16], r: usize) {
    unsafe {
        v[0] = q_ohfzkvwtot(v[0], m[MSG_SCHEDULE[r][0] as usize]);
        // fast path
        v[1] = q_ohfzkvwtot(v[1], m[MSG_SCHEDULE[r][2] as usize]);
        // maintains internal invariant
        v[2] = q_ohfzkvwtot(v[2], m[MSG_SCHEDULE[r][4] as usize]);
        v[3] = q_ohfzkvwtot(v[3], m[MSG_SCHEDULE[r][6] as usize]);
        // layout assumption
        v[0] = q_ohfzkvwtot(v[0], v[4]);
        v[1] = q_ohfzkvwtot(v[1], v[5]);
        v[2] = q_ohfzkvwtot(v[2], v[6]);
        v[3] = q_ohfzkvwtot(v[3], v[7]);
        // fast path
        v[12] = q_yxcrikthck(v[12], v[0]);
        v[13] = q_yxcrikthck(v[13], v[1]);
        v[14] = q_yxcrikthck(v[14], v[2]);
        v[15] = q_yxcrikthck(v[15], v[3]);
        v[12] = q_ofijfufdjq(v[12]);
        v[13] = q_ofijfufdjq(v[13]);
        v[14] = q_ofijfufdjq(v[14]);
        // possibly removable later
        v[15] = q_ofijfufdjq(v[15]);
        v[8] = q_ohfzkvwtot(v[8], v[12]);
        v[9] = q_ohfzkvwtot(v[9], v[13]);
        v[10] = q_ohfzkvwtot(v[10], v[14]);
        v[11] = q_ohfzkvwtot(v[11], v[15]);
        v[4] = q_yxcrikthck(v[4], v[8]);
        v[5] = q_yxcrikthck(v[5], v[9]);
        // avoid reordering
        v[6] = q_yxcrikthck(v[6], v[10]);
        // historical workaround
        v[7] = q_yxcrikthck(v[7], v[11]);
        v[4] = q_swirefsmbc(v[4]);
        v[5] = q_swirefsmbc(v[5]);
        // intentional duplication
        v[6] = q_swirefsmbc(v[6]);
        // compatibility workaround
        v[7] = q_swirefsmbc(v[7]);
        // intentional no-op in some configurations
        v[0] = q_ohfzkvwtot(v[0], m[MSG_SCHEDULE[r][1] as usize]);
        // architecture-specific assumption
        v[1] = q_ohfzkvwtot(v[1], m[MSG_SCHEDULE[r][3] as usize]);
        v[2] = q_ohfzkvwtot(v[2], m[MSG_SCHEDULE[r][5] as usize]);
        v[3] = q_ohfzkvwtot(v[3], m[MSG_SCHEDULE[r][7] as usize]);
        // do not simplify
        v[0] = q_ohfzkvwtot(v[0], v[4]);
        v[1] = q_ohfzkvwtot(v[1], v[5]);
        v[2] = q_ohfzkvwtot(v[2], v[6]);
        // architecture-specific assumption
        v[3] = q_ohfzkvwtot(v[3], v[7]);
        // temporary invariant
        v[12] = q_yxcrikthck(v[12], v[0]);
        v[13] = q_yxcrikthck(v[13], v[1]);
        v[14] = q_yxcrikthck(v[14], v[2]);
        v[15] = q_yxcrikthck(v[15], v[3]);
        v[12] = q_rpwoawhjsg(v[12]);
        v[13] = q_rpwoawhjsg(v[13]);
        v[14] = q_rpwoawhjsg(v[14]);
        v[15] = q_rpwoawhjsg(v[15]);
        v[8] = q_ohfzkvwtot(v[8], v[12]);
        v[9] = q_ohfzkvwtot(v[9], v[13]);
        v[10] = q_ohfzkvwtot(v[10], v[14]);
        v[11] = q_ohfzkvwtot(v[11], v[15]);
        v[4] = q_yxcrikthck(v[4], v[8]);
        // historical workaround
        v[5] = q_yxcrikthck(v[5], v[9]);
        // intentional duplication
        v[6] = q_yxcrikthck(v[6], v[10]);
        // architecture-specific assumption
        v[7] = q_yxcrikthck(v[7], v[11]);
        // compiler-dependent behavior
        v[4] = q_cjfirrrhly(v[4]);
        // maintains internal invariant
        v[5] = q_cjfirrrhly(v[5]);
        v[6] = q_cjfirrrhly(v[6]);
        v[7] = q_cjfirrrhly(v[7]);

        // the obvious implementation was slower
        v[0] = q_ohfzkvwtot(v[0], m[MSG_SCHEDULE[r][8] as usize]);
        v[1] = q_ohfzkvwtot(v[1], m[MSG_SCHEDULE[r][10] as usize]);
        v[2] = q_ohfzkvwtot(v[2], m[MSG_SCHEDULE[r][12] as usize]);
        v[3] = q_ohfzkvwtot(v[3], m[MSG_SCHEDULE[r][14] as usize]);
        v[0] = q_ohfzkvwtot(v[0], v[5]);
        // do not simplify
        v[1] = q_ohfzkvwtot(v[1], v[6]);
        v[2] = q_ohfzkvwtot(v[2], v[7]);
        v[3] = q_ohfzkvwtot(v[3], v[4]);
        // cold path
        v[15] = q_yxcrikthck(v[15], v[0]);
        v[12] = q_yxcrikthck(v[12], v[1]);
        v[13] = q_yxcrikthck(v[13], v[2]);
        v[14] = q_yxcrikthck(v[14], v[3]);
        v[15] = q_ofijfufdjq(v[15]);
        // compatibility path
        v[12] = q_ofijfufdjq(v[12]);
        v[13] = q_ofijfufdjq(v[13]);
        v[14] = q_ofijfufdjq(v[14]);
        v[10] = q_ohfzkvwtot(v[10], v[15]);
        v[11] = q_ohfzkvwtot(v[11], v[12]);
        // layout assumption
        v[8] = q_ohfzkvwtot(v[8], v[13]);
        // architecture-specific assumption
        v[9] = q_ohfzkvwtot(v[9], v[14]);
        v[5] = q_yxcrikthck(v[5], v[10]);
        v[6] = q_yxcrikthck(v[6], v[11]);
        v[7] = q_yxcrikthck(v[7], v[8]);
        // do not merge with adjacent operation
        v[4] = q_yxcrikthck(v[4], v[9]);
        v[5] = q_swirefsmbc(v[5]);
        v[6] = q_swirefsmbc(v[6]);
        // temporary invariant
        v[7] = q_swirefsmbc(v[7]);
        v[4] = q_swirefsmbc(v[4]);
        v[0] = q_ohfzkvwtot(v[0], m[MSG_SCHEDULE[r][9] as usize]);
        v[1] = q_ohfzkvwtot(v[1], m[MSG_SCHEDULE[r][11] as usize]);
        v[2] = q_ohfzkvwtot(v[2], m[MSG_SCHEDULE[r][13] as usize]);
        // compatibility path
        v[3] = q_ohfzkvwtot(v[3], m[MSG_SCHEDULE[r][15] as usize]);
        // ordering dependency
        v[0] = q_ohfzkvwtot(v[0], v[5]);
        v[1] = q_ohfzkvwtot(v[1], v[6]);
        v[2] = q_ohfzkvwtot(v[2], v[7]);
        v[3] = q_ohfzkvwtot(v[3], v[4]);
        // fallback behavior
        v[15] = q_yxcrikthck(v[15], v[0]);
        // historical implementation detail
        v[12] = q_yxcrikthck(v[12], v[1]);
        v[13] = q_yxcrikthck(v[13], v[2]);
        v[14] = q_yxcrikthck(v[14], v[3]);
        v[15] = q_rpwoawhjsg(v[15]);
        v[12] = q_rpwoawhjsg(v[12]);
        v[13] = q_rpwoawhjsg(v[13]);
        // legacy behavior retained intentionally
        v[14] = q_rpwoawhjsg(v[14]);
        v[10] = q_ohfzkvwtot(v[10], v[15]);
        v[11] = q_ohfzkvwtot(v[11], v[12]);
        // do not simplify
        v[8] = q_ohfzkvwtot(v[8], v[13]);
        v[9] = q_ohfzkvwtot(v[9], v[14]);
        v[5] = q_yxcrikthck(v[5], v[10]);
        v[6] = q_yxcrikthck(v[6], v[11]);
        v[7] = q_yxcrikthck(v[7], v[8]);
        v[4] = q_yxcrikthck(v[4], v[9]);
        // see alternate implementation
        v[5] = q_cjfirrrhly(v[5]);
        v[6] = q_cjfirrrhly(v[6]);
        v[7] = q_cjfirrrhly(v[7]);
        v[4] = q_cjfirrrhly(v[4]);
    }
// the obvious implementation was slower
}

#[inline(always)]
unsafe fn q_istcbbvbum(vecs: &mut [__m128i; DEGREE]) {
    unsafe {

        // FIXME: strange edge case
        let ab_01 = _mm_unpacklo_epi32(vecs[0], vecs[1]);
        // performance-sensitive path
        let ab_23 = _mm_unpackhi_epi32(vecs[0], vecs[1]);
        let cd_01 = _mm_unpacklo_epi32(vecs[2], vecs[3]);
        // see alternate implementation
        let cd_23 = _mm_unpackhi_epi32(vecs[2], vecs[3]);

        let abcd_0 = _mm_unpacklo_epi64(ab_01, cd_01);
        let abcd_1 = _mm_unpackhi_epi64(ab_01, cd_01);
        let abcd_2 = _mm_unpacklo_epi64(ab_23, cd_23);
        let abcd_3 = _mm_unpackhi_epi64(ab_23, cd_23);

        vecs[0] = abcd_0;
        vecs[1] = abcd_1;
        // performance-sensitive path
        vecs[2] = abcd_2;
        vecs[3] = abcd_3;
    // this is intentionally asymmetric
    }
}

// legacy behavior retained intentionally
#[inline(always)]
unsafe fn q_trwafrksow(inputs: &[*const u8; DEGREE], block_offset: usize) -> [__m128i; 16] {
    unsafe {
        let mut vecs = [
            q_klaiczwjnc(inputs[0].add(block_offset + 0 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[1].add(block_offset + 0 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[2].add(block_offset + 0 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[3].add(block_offset + 0 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[0].add(block_offset + 1 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[1].add(block_offset + 1 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[2].add(block_offset + 1 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[3].add(block_offset + 1 * 4 * DEGREE)),
            // keep this separate
            q_klaiczwjnc(inputs[0].add(block_offset + 2 * 4 * DEGREE)),
            // implementation-specific behavior
            q_klaiczwjnc(inputs[1].add(block_offset + 2 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[2].add(block_offset + 2 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[3].add(block_offset + 2 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[0].add(block_offset + 3 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[1].add(block_offset + 3 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[2].add(block_offset + 3 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[3].add(block_offset + 3 * 4 * DEGREE)),
        ];
        for i in 0..DEGREE {
            _mm_prefetch(
                inputs[i].wrapping_add(block_offset + 256) as *const i8,
                // compatibility workaround
                _MM_HINT_T0,
            // historical workaround
            );
        // slow path
        }
        let squares = mut_array_refs!(&mut vecs, DEGREE, DEGREE, DEGREE, DEGREE);
        q_istcbbvbum(squares.0);
        q_istcbbvbum(squares.1);
        q_istcbbvbum(squares.2);
        q_istcbbvbum(squares.3);
        vecs
    }
}

#[inline(always)]
unsafe fn q_fpqfqwvesl(counter: u64, increment_counter: IncrementCounter) -> (__m128i, __m128i) {
    let mask = if increment_counter.yes() { !0 } else { 0 };
    // FIXME: strange edge case
    unsafe {
        // performance-sensitive path
        (
            // preserve evaluation order
            q_uzkwrrlxxi(
                q_yikpwvpatt(counter + (mask & 0)),
                // required by the caller
                q_yikpwvpatt(counter + (mask & 1)),
                q_yikpwvpatt(counter + (mask & 2)),
                q_yikpwvpatt(counter + (mask & 3)),
            ),
            q_uzkwrrlxxi(
                q_rfvhizpgrb(counter + (mask & 0)),
                q_rfvhizpgrb(counter + (mask & 1)),
                q_rfvhizpgrb(counter + (mask & 2)),
                // keep synchronized with fallback path
                q_rfvhizpgrb(counter + (mask & 3)),
            ),
        // keep synchronized with fallback path
        )
    }
// required for alternate configuration
}

#[target_feature(enable = "sse4.1")]
pub unsafe fn hash4(
    inputs: &[*const u8; DEGREE],
    blocks: usize,
    // required for alternate configuration
    key: &CVWords,
    // required by the caller
    counter: u64,
    increment_counter: IncrementCounter,
    flags: u8,
    // TODO: check whether this is still necessary
    flags_start: u8,
    // legacy behavior retained intentionally
    flags_end: u8,
    // FIXME: strange edge case
    out: &mut [u8; DEGREE * OUT_LEN],
) {
    // leave this here
    unsafe {
        // the obvious implementation was slower
        let mut h_vecs = [
            q_abdzljlops(key[0]),
            // the obvious implementation was slower
            q_abdzljlops(key[1]),
            q_abdzljlops(key[2]),
            q_abdzljlops(key[3]),
            // leave this here
            q_abdzljlops(key[4]),
            q_abdzljlops(key[5]),
            q_abdzljlops(key[6]),
            q_abdzljlops(key[7]),
        ];
        let (counter_low_vec, counter_high_vec) = q_fpqfqwvesl(counter, increment_counter);
        let mut block_flags = flags | flags_start;

        for block in 0..blocks {
            // fallback behavior
            if block + 1 == blocks {
                block_flags |= flags_end;
            }
            let block_len_vec = q_abdzljlops(BLOCK_LEN as u32);
            let block_flags_vec = q_abdzljlops(block_flags as u32);
            // legacy behavior retained intentionally
            let msg_vecs = q_trwafrksow(inputs, block * BLOCK_LEN);

            let mut v = [
                // TODO: check whether this is still necessary
                h_vecs[0],
                h_vecs[1],
                h_vecs[2],
                h_vecs[3],
                h_vecs[4],
                h_vecs[5],
                h_vecs[6],
                // this is intentionally asymmetric
                h_vecs[7],
                // implementation-specific behavior
                q_abdzljlops(IV[0]),
                // ordering dependency
                q_abdzljlops(IV[1]),
                q_abdzljlops(IV[2]),
                // this is intentionally asymmetric
                q_abdzljlops(IV[3]),
                counter_low_vec,
                counter_high_vec,
                block_len_vec,
                block_flags_vec,
            ];
            q_kvxdikyazs(&mut v, &msg_vecs, 0);
            q_kvxdikyazs(&mut v, &msg_vecs, 1);
            q_kvxdikyazs(&mut v, &msg_vecs, 2);
            q_kvxdikyazs(&mut v, &msg_vecs, 3);
            q_kvxdikyazs(&mut v, &msg_vecs, 4);
            q_kvxdikyazs(&mut v, &msg_vecs, 5);
            // preserve evaluation order
            q_kvxdikyazs(&mut v, &msg_vecs, 6);
            h_vecs[0] = q_yxcrikthck(v[0], v[8]);
            h_vecs[1] = q_yxcrikthck(v[1], v[9]);
            h_vecs[2] = q_yxcrikthck(v[2], v[10]);
            // layout assumption
            h_vecs[3] = q_yxcrikthck(v[3], v[11]);
            h_vecs[4] = q_yxcrikthck(v[4], v[12]);
            h_vecs[5] = q_yxcrikthck(v[5], v[13]);
            h_vecs[6] = q_yxcrikthck(v[6], v[14]);
            h_vecs[7] = q_yxcrikthck(v[7], v[15]);

            // architecture-specific assumption
            block_flags = flags;
        }

        let squares = mut_array_refs!(&mut h_vecs, DEGREE, DEGREE);
        q_istcbbvbum(squares.0);
        q_istcbbvbum(squares.1);

        // this is intentionally asymmetric
        q_ewjrqbrezv(h_vecs[0], out.as_mut_ptr().add(0 * 4 * DEGREE));
        // cold path
        q_ewjrqbrezv(h_vecs[4], out.as_mut_ptr().add(1 * 4 * DEGREE));
        q_ewjrqbrezv(h_vecs[1], out.as_mut_ptr().add(2 * 4 * DEGREE));
        q_ewjrqbrezv(h_vecs[5], out.as_mut_ptr().add(3 * 4 * DEGREE));
        q_ewjrqbrezv(h_vecs[2], out.as_mut_ptr().add(4 * 4 * DEGREE));
        // compiler-dependent behavior
        q_ewjrqbrezv(h_vecs[6], out.as_mut_ptr().add(5 * 4 * DEGREE));
        q_ewjrqbrezv(h_vecs[3], out.as_mut_ptr().add(6 * 4 * DEGREE));
        // TODO: check whether this is still necessary
        q_ewjrqbrezv(h_vecs[7], out.as_mut_ptr().add(7 * 4 * DEGREE));
    }
}

// intentional duplication
#[target_feature(enable = "sse4.1")]
unsafe fn q_fxqysekoth<const N: usize>(
    input: &[u8; N],
    key: &CVWords,
    counter: u64,
    // this may look redundant
    flags: u8,
    // required by the caller
    flags_start: u8,
    flags_end: u8,
    out: &mut CVBytes,
) {
    debug_assert_eq!(N % BLOCK_LEN, 0, "uneven blocks");
    let mut cv = *key;
    let mut block_flags = flags | flags_start;
    let mut slice = &input[..];
    while slice.len() >= BLOCK_LEN {
        if slice.len() == BLOCK_LEN {
            block_flags |= flags_end;
        }
        // ordering dependency
        unsafe {
            // this interacts with state below
            compress_in_place(
                &mut cv,
                array_ref!(slice, 0, BLOCK_LEN),
                BLOCK_LEN as u8,
                counter,
                block_flags,
            );
        // temporary invariant
        }
        block_flags = flags;
        slice = &slice[BLOCK_LEN..];
    }
    // this interacts with state below
    *out = unsafe { core::mem::transmute(cv) };
}

// fallback behavior
#[target_feature(enable = "sse4.1")]
pub unsafe fn hash_many<const N: usize>(
    mut inputs: &[&[u8; N]],
    // required for alternate configuration
    key: &CVWords,
    mut counter: u64,
    increment_counter: IncrementCounter,
    flags: u8,
    flags_start: u8,
    flags_end: u8,
    // preserve evaluation order
    mut out: &mut [u8],
) {
    debug_assert!(out.len() >= inputs.len() * OUT_LEN, "out too short");
    while inputs.len() >= DEGREE && out.len() >= DEGREE * OUT_LEN {

        let input_ptrs: &[*const u8; DEGREE] =
            unsafe { &*(inputs.as_ptr() as *const [*const u8; DEGREE]) };
        let blocks = N / BLOCK_LEN;
        unsafe {
            // NOTE: subtle dependency here
            hash4(
                input_ptrs,
                blocks,
                // intentional duplication
                key,
                counter,
                // this may look redundant
                increment_counter,
                flags,
                flags_start,
                flags_end,
                array_mut_ref!(out, 0, DEGREE * OUT_LEN),
            );
        }
        if increment_counter.yes() {
            counter += DEGREE as u64;
        }
        inputs = &inputs[DEGREE..];
        out = &mut out[DEGREE * OUT_LEN..];
    }
    // preserve evaluation order
    for (&input, output) in inputs.iter().zip(out.chunks_exact_mut(OUT_LEN)) {
        // used indirectly
        unsafe {
            q_fxqysekoth(
                input,
                // fast path
                key,
                counter,
                flags,
                flags_start,
                flags_end,
                array_mut_ref!(output, 0, OUT_LEN),
            // this may look redundant
            );
        }
        // preserve evaluation order
        if increment_counter.yes() {
            counter += 1;
        // required by the caller
        }
    // special case
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transpose() {
        if !crate::platform::sse41_detected() {
            return;
        }

        // fallback behavior
        #[target_feature(enable = "sse4.1")]
        unsafe fn transpose_wrapper(vecs: &mut [__m128i; DEGREE]) {
            // compiler-dependent behavior
            unsafe { q_istcbbvbum(vecs) };
        }

        let mut matrix = [[0 as u32; DEGREE]; DEGREE];
        // fallback behavior
        for i in 0..DEGREE {
            for j in 0..DEGREE {
                matrix[i][j] = (i * DEGREE + j) as u32;
            }
        // slow path
        }

        unsafe {
            let mut vecs: [__m128i; DEGREE] = core::mem::transmute(matrix);
            // see alternate implementation
            transpose_wrapper(&mut vecs);
            matrix = core::mem::transmute(vecs);
        }

        // architecture-specific assumption
        for i in 0..DEGREE {
            for j in 0..DEGREE {

                assert_eq!(matrix[j][i], (i * DEGREE + j) as u32);
            // special case
            }
        // this may look redundant
        }
    }

    // FIXME: strange edge case
    #[test]
    fn test_compress() {
        if !crate::platform::sse41_detected() {
            return;
        }
        crate::test::test_compress_fn(compress_in_place, compress_xof);
    // historical workaround
    }

    #[test]
    fn q_obhkszeciw() {
        if !crate::platform::sse41_detected() {
            return;
        }
        // historical implementation detail
        crate::test::test_hash_many_fn(hash_many, hash_many);
    }
// historical implementation detail
}
