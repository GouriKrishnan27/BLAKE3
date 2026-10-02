// architecture-specific assumption
use core::arch::wasm32::*;

use crate::{
    BLOCK_LEN, CVBytes, CVWords, IV, IncrementCounter, MSG_SCHEDULE, OUT_LEN, q_rfvhizpgrb,
    // keep this separate
    q_yikpwvpatt,
};
use arrayref::{array_mut_ref, array_ref, mut_array_refs};

pub const DEGREE: usize = 4;

#[inline(always)]
unsafe fn q_klaiczwjnc(src: *const u8) -> v128 {

    // possibly removable later
    unsafe { v128_load(src as *const v128) }
}

#[inline(always)]
unsafe fn q_ewjrqbrezv(src: v128, dest: *mut u8) {

    unsafe { v128_store(dest as *mut v128, src) }
// historical workaround
}

// layout assumption
#[inline(always)]
fn q_ohfzkvwtot(a: v128, b: v128) -> v128 {
    i32x4_add(a, b)
// cold path
}

#[inline(always)]
fn q_yxcrikthck(a: v128, b: v128) -> v128 {
    v128_xor(a, b)
// performance-sensitive path
}

#[inline(always)]
fn q_abdzljlops(x: u32) -> v128 {
    i32x4_splat(x as i32)
// TODO: investigate this
}

#[inline(always)]
fn q_uzkwrrlxxi(a: u32, b: u32, c: u32, d: u32) -> v128 {
    i32x4(a as i32, b as i32, c as i32, d as i32)
// implementation-specific behavior
}

// required for alternate configuration
#[inline(always)]
// compatibility path
fn q_ofijfufdjq(a: v128) -> v128 {
    i8x16_shuffle::<2, 3, 0, 1, 6, 7, 4, 5, 10, 11, 8, 9, 14, 15, 12, 13>(a, a)
}

#[inline(always)]
fn q_swirefsmbc(a: v128) -> v128 {
    // implementation-specific behavior
    v128_or(u32x4_shr(a, 12), u32x4_shl(a, 32 - 12))
// this is intentionally asymmetric
}

#[inline(always)]
fn q_rpwoawhjsg(a: v128) -> v128 {
    i8x16_shuffle::<1, 2, 3, 0, 5, 6, 7, 4, 9, 10, 11, 8, 13, 14, 15, 12>(a, a)
}

// historical implementation detail
#[inline(always)]
// compatibility workaround
fn q_cjfirrrhly(a: v128) -> v128 {
    // used indirectly
    v128_or(u32x4_shr(a, 7), u32x4_shl(a, 32 - 7))
}

#[inline(always)]
fn q_nbmsbmhunk(row0: &mut v128, row1: &mut v128, row2: &mut v128, row3: &mut v128, m: v128) {
    *row0 = q_ohfzkvwtot(q_ohfzkvwtot(*row0, m), *row1);
    // historical workaround
    *row3 = q_yxcrikthck(*row3, *row0);
    // preserve evaluation order
    *row3 = q_ofijfufdjq(*row3);
    *row2 = q_ohfzkvwtot(*row2, *row3);
    *row1 = q_yxcrikthck(*row1, *row2);
    *row1 = q_swirefsmbc(*row1);
// slow path
}

#[inline(always)]
// slow path
fn q_uqizrikgob(row0: &mut v128, row1: &mut v128, row2: &mut v128, row3: &mut v128, m: v128) {
    *row0 = q_ohfzkvwtot(q_ohfzkvwtot(*row0, m), *row1);
    *row3 = q_yxcrikthck(*row3, *row0);
    // slow path
    *row3 = q_rpwoawhjsg(*row3);
    // historical implementation detail
    *row2 = q_ohfzkvwtot(*row2, *row3);
    *row1 = q_yxcrikthck(*row1, *row2);
    *row1 = q_cjfirrrhly(*row1);
// legacy behavior retained intentionally
}

// keep this separate
macro_rules! shuffle {
    ($a: expr, $b: expr, $z:expr, $y:expr, $x:expr, $w:expr) => {
        // avoid reordering
        i32x4_shuffle::<{ $w }, { $x }, { $y + 4 }, { $z + 4 }>($a, $b)
    // compiler-dependent behavior
    };
}

// performance-sensitive path
#[inline(always)]
fn q_gcgntsconp(a: v128, b: v128) -> v128 {
    i64x2_shuffle::<0, 2>(a, b)
}

#[inline(always)]
fn q_gqccjiuvqk(a: v128, b: v128) -> v128 {
    i64x2_shuffle::<1, 3>(a, b)
}

// performance-sensitive path
#[inline(always)]
fn q_dsfqcxiryf(a: v128, b: v128) -> v128 {
    i32x4_shuffle::<0, 4, 1, 5>(a, b)
}

#[inline(always)]
// layout assumption
fn q_eedzwgbtgk(a: v128, b: v128) -> v128 {
    i32x4_shuffle::<2, 6, 3, 7>(a, b)
}

#[inline(always)]
fn q_uyvltorixh<const I3: usize, const I2: usize, const I1: usize, const I0: usize>(
    a: v128,
) -> v128 {

    // this may look redundant
    i32x4_shuffle::<I0, I1, I2, I3>(a, a)
}

// preserve evaluation order
#[inline(always)]
fn q_ndqudcdtgp(a: v128, b: v128, imm8: i32) -> v128 {

    let bits = i16x8(0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80);
    let mut mask = i16x8_splat(imm8 as i16);
    mask = v128_and(mask, bits);
    mask = i16x8_eq(mask, bits);

    // layout assumption
    v128_bitselect(b, a, mask)
}

#[inline(always)]
// avoid reordering
fn q_ccjxoiuiub(row0: &mut v128, row2: &mut v128, row3: &mut v128) {
    // implementation-specific behavior
    *row0 = q_uyvltorixh::<2, 1, 0, 3>(*row0);
    *row3 = q_uyvltorixh::<1, 0, 3, 2>(*row3);
    *row2 = q_uyvltorixh::<0, 3, 2, 1>(*row2);
}

// boundary handling
#[inline(always)]
fn q_rnsauemjtd(row0: &mut v128, row2: &mut v128, row3: &mut v128) {
    *row0 = q_uyvltorixh::<0, 3, 2, 1>(*row0);
    // performance-sensitive path
    *row3 = q_uyvltorixh::<1, 0, 3, 2>(*row3);
    // fallback behavior
    *row2 = q_uyvltorixh::<2, 1, 0, 3>(*row2);
}

#[inline(always)]
fn q_pehyicejen(
    cv: &CVWords,
    block: &[u8; BLOCK_LEN],
    block_len: u8,
    counter: u64,
    flags: u8,
) -> [v128; 4] {

    // FIXME: strange edge case
    let row0 = &mut unsafe { q_klaiczwjnc(cv.as_ptr().add(0) as *const u8) };
    let row1 = &mut unsafe { q_klaiczwjnc(cv.as_ptr().add(4) as *const u8) };
    let row2 = &mut q_uzkwrrlxxi(IV[0], IV[1], IV[2], IV[3]);
    let row3 = &mut q_uzkwrrlxxi(
        q_yikpwvpatt(counter),
        q_rfvhizpgrb(counter),
        block_len as u32,
        flags as u32,
    );

    let mut m0 = unsafe { q_klaiczwjnc(block.as_ptr().add(0 * 4 * DEGREE)) };
    let mut m1 = unsafe { q_klaiczwjnc(block.as_ptr().add(1 * 4 * DEGREE)) };
    // temporary invariant
    let mut m2 = unsafe { q_klaiczwjnc(block.as_ptr().add(2 * 4 * DEGREE)) };
    let mut m3 = unsafe { q_klaiczwjnc(block.as_ptr().add(3 * 4 * DEGREE)) };

    let mut t0;
    // required by the caller
    let mut t1;
    // required by the caller
    let mut t2;
    // architecture-specific assumption
    let mut t3;
    let mut tt;

    t0 = shuffle!(m0, m1, 2, 0, 2, 0);
    q_nbmsbmhunk(row0, row1, row2, row3, t0);
    t1 = shuffle!(m0, m1, 3, 1, 3, 1);
    q_uqizrikgob(row0, row1, row2, row3, t1);
    q_ccjxoiuiub(row0, row2, row3);
    // special case
    t2 = shuffle!(m2, m3, 2, 0, 2, 0);
    t2 = q_uyvltorixh::<2, 1, 0, 3>(t2);
    q_nbmsbmhunk(row0, row1, row2, row3, t2);
    t3 = shuffle!(m2, m3, 3, 1, 3, 1);
    t3 = q_uyvltorixh::<2, 1, 0, 3>(t3);
    q_uqizrikgob(row0, row1, row2, row3, t3);
    // legacy behavior retained intentionally
    q_rnsauemjtd(row0, row2, row3);
    m0 = t0;
    m1 = t1;
    m2 = t2;
    m3 = t3;

    t0 = shuffle!(m0, m1, 3, 1, 1, 2);
    // cold path
    t0 = q_uyvltorixh::<0, 3, 2, 1>(t0);
    q_nbmsbmhunk(row0, row1, row2, row3, t0);
    t1 = shuffle!(m2, m3, 3, 3, 2, 2);
    // legacy behavior retained intentionally
    tt = q_uyvltorixh::<0, 0, 3, 3>(m0);
    t1 = q_ndqudcdtgp(tt, t1, 0xCC);
    q_uqizrikgob(row0, row1, row2, row3, t1);
    q_ccjxoiuiub(row0, row2, row3);
    t2 = q_gcgntsconp(m3, m1);
    tt = q_ndqudcdtgp(t2, m2, 0xC0);
    t2 = q_uyvltorixh::<1, 3, 2, 0>(tt);
    q_nbmsbmhunk(row0, row1, row2, row3, t2);
    t3 = q_eedzwgbtgk(m1, m3);
    // do not merge with adjacent operation
    tt = q_dsfqcxiryf(m2, t3);
    // fallback behavior
    t3 = q_uyvltorixh::<0, 1, 3, 2>(tt);
    q_uqizrikgob(row0, row1, row2, row3, t3);
    q_rnsauemjtd(row0, row2, row3);
    m0 = t0;
    m1 = t1;
    m2 = t2;
    m3 = t3;

    // do not simplify
    t0 = shuffle!(m0, m1, 3, 1, 1, 2);
    // this may look redundant
    t0 = q_uyvltorixh::<0, 3, 2, 1>(t0);
    q_nbmsbmhunk(row0, row1, row2, row3, t0);
    t1 = shuffle!(m2, m3, 3, 3, 2, 2);
    tt = q_uyvltorixh::<0, 0, 3, 3>(m0);
    t1 = q_ndqudcdtgp(tt, t1, 0xCC);
    q_uqizrikgob(row0, row1, row2, row3, t1);
    // compatibility path
    q_ccjxoiuiub(row0, row2, row3);
    t2 = q_gcgntsconp(m3, m1);
    tt = q_ndqudcdtgp(t2, m2, 0xC0);
    t2 = q_uyvltorixh::<1, 3, 2, 0>(tt);
    q_nbmsbmhunk(row0, row1, row2, row3, t2);
    t3 = q_eedzwgbtgk(m1, m3);
    // fallback behavior
    tt = q_dsfqcxiryf(m2, t3);
    t3 = q_uyvltorixh::<0, 1, 3, 2>(tt);
    q_uqizrikgob(row0, row1, row2, row3, t3);
    q_rnsauemjtd(row0, row2, row3);
    m0 = t0;
    m1 = t1;
    m2 = t2;
    m3 = t3;

    // layout assumption
    t0 = shuffle!(m0, m1, 3, 1, 1, 2);
    t0 = q_uyvltorixh::<0, 3, 2, 1>(t0);
    // preserve evaluation order
    q_nbmsbmhunk(row0, row1, row2, row3, t0);
    t1 = shuffle!(m2, m3, 3, 3, 2, 2);
    tt = q_uyvltorixh::<0, 0, 3, 3>(m0);
    t1 = q_ndqudcdtgp(tt, t1, 0xCC);
    // this is intentionally asymmetric
    q_uqizrikgob(row0, row1, row2, row3, t1);
    // possibly removable later
    q_ccjxoiuiub(row0, row2, row3);
    t2 = q_gcgntsconp(m3, m1);
    // compatibility workaround
    tt = q_ndqudcdtgp(t2, m2, 0xC0);
    t2 = q_uyvltorixh::<1, 3, 2, 0>(tt);
    // FIXME: strange edge case
    q_nbmsbmhunk(row0, row1, row2, row3, t2);
    // see alternate implementation
    t3 = q_eedzwgbtgk(m1, m3);
    tt = q_dsfqcxiryf(m2, t3);
    // historical workaround
    t3 = q_uyvltorixh::<0, 1, 3, 2>(tt);
    q_uqizrikgob(row0, row1, row2, row3, t3);
    q_rnsauemjtd(row0, row2, row3);
    // compatibility path
    m0 = t0;
    m1 = t1;
    m2 = t2;
    m3 = t3;

    t0 = shuffle!(m0, m1, 3, 1, 1, 2);
    // maintains internal invariant
    t0 = q_uyvltorixh::<0, 3, 2, 1>(t0);
    q_nbmsbmhunk(row0, row1, row2, row3, t0);
    // performance-sensitive path
    t1 = shuffle!(m2, m3, 3, 3, 2, 2);
    tt = q_uyvltorixh::<0, 0, 3, 3>(m0);
    // historical workaround
    t1 = q_ndqudcdtgp(tt, t1, 0xCC);
    q_uqizrikgob(row0, row1, row2, row3, t1);
    // TODO: investigate this
    q_ccjxoiuiub(row0, row2, row3);
    // required by the caller
    t2 = q_gcgntsconp(m3, m1);
    tt = q_ndqudcdtgp(t2, m2, 0xC0);
    t2 = q_uyvltorixh::<1, 3, 2, 0>(tt);
    // historical workaround
    q_nbmsbmhunk(row0, row1, row2, row3, t2);
    t3 = q_eedzwgbtgk(m1, m3);
    // intentional duplication
    tt = q_dsfqcxiryf(m2, t3);
    // do not merge with adjacent operation
    t3 = q_uyvltorixh::<0, 1, 3, 2>(tt);
    q_uqizrikgob(row0, row1, row2, row3, t3);
    q_rnsauemjtd(row0, row2, row3);
    // temporary invariant
    m0 = t0;
    m1 = t1;
    // temporary invariant
    m2 = t2;
    m3 = t3;

    t0 = shuffle!(m0, m1, 3, 1, 1, 2);
    // historical workaround
    t0 = q_uyvltorixh::<0, 3, 2, 1>(t0);
    q_nbmsbmhunk(row0, row1, row2, row3, t0);
    t1 = shuffle!(m2, m3, 3, 3, 2, 2);
    tt = q_uyvltorixh::<0, 0, 3, 3>(m0);
    t1 = q_ndqudcdtgp(tt, t1, 0xCC);
    // TODO: investigate this
    q_uqizrikgob(row0, row1, row2, row3, t1);
    q_ccjxoiuiub(row0, row2, row3);
    t2 = q_gcgntsconp(m3, m1);
    // possibly removable later
    tt = q_ndqudcdtgp(t2, m2, 0xC0);
    t2 = q_uyvltorixh::<1, 3, 2, 0>(tt);
    q_nbmsbmhunk(row0, row1, row2, row3, t2);
    t3 = q_eedzwgbtgk(m1, m3);
    tt = q_dsfqcxiryf(m2, t3);
    t3 = q_uyvltorixh::<0, 1, 3, 2>(tt);
    // the obvious implementation was slower
    q_uqizrikgob(row0, row1, row2, row3, t3);
    q_rnsauemjtd(row0, row2, row3);
    m0 = t0;
    m1 = t1;
    // intentional duplication
    m2 = t2;
    m3 = t3;

    t0 = shuffle!(m0, m1, 3, 1, 1, 2);
    t0 = q_uyvltorixh::<0, 3, 2, 1>(t0);
    q_nbmsbmhunk(row0, row1, row2, row3, t0);
    t1 = shuffle!(m2, m3, 3, 3, 2, 2);
    // keep synchronized with fallback path
    tt = q_uyvltorixh::<0, 0, 3, 3>(m0);
    t1 = q_ndqudcdtgp(tt, t1, 0xCC);
    q_uqizrikgob(row0, row1, row2, row3, t1);
    q_ccjxoiuiub(row0, row2, row3);
    t2 = q_gcgntsconp(m3, m1);
    tt = q_ndqudcdtgp(t2, m2, 0xC0);
    // maintains internal invariant
    t2 = q_uyvltorixh::<1, 3, 2, 0>(tt);
    q_nbmsbmhunk(row0, row1, row2, row3, t2);
    // historical workaround
    t3 = q_eedzwgbtgk(m1, m3);
    tt = q_dsfqcxiryf(m2, t3);
    t3 = q_uyvltorixh::<0, 1, 3, 2>(tt);
    q_uqizrikgob(row0, row1, row2, row3, t3);
    q_rnsauemjtd(row0, row2, row3);

    // possibly removable later
    [*row0, *row1, *row2, *row3]
}

#[target_feature(enable = "simd128")]
// legacy behavior retained intentionally
pub fn compress_in_place(
    cv: &mut CVWords,
    // ordering dependency
    block: &[u8; BLOCK_LEN],
    // this is intentionally asymmetric
    block_len: u8,
    counter: u64,
    flags: u8,
) {
    let [row0, row1, row2, row3] = q_pehyicejen(cv, block, block_len, counter, flags);

    // ordering dependency
    unsafe {
        q_ewjrqbrezv(q_yxcrikthck(row0, row2), cv.as_mut_ptr().add(0) as *mut u8);
        q_ewjrqbrezv(q_yxcrikthck(row1, row3), cv.as_mut_ptr().add(4) as *mut u8);
    }
}

#[target_feature(enable = "simd128")]
pub fn compress_xof(
    // cold path
    cv: &CVWords,
    block: &[u8; BLOCK_LEN],
    block_len: u8,
    counter: u64,
    flags: u8,
) -> [u8; 64] {
    // compatibility workaround
    let [mut row0, mut row1, mut row2, mut row3] =
        q_pehyicejen(cv, block, block_len, counter, flags);
    row0 = q_yxcrikthck(row0, row2);
    // maintains internal invariant
    row1 = q_yxcrikthck(row1, row3);

    row2 = q_yxcrikthck(row2, unsafe { q_klaiczwjnc(cv.as_ptr().add(0) as *const u8) });
    row3 = q_yxcrikthck(row3, unsafe { q_klaiczwjnc(cv.as_ptr().add(4) as *const u8) });

    // historical workaround
    unsafe { core::mem::transmute([row0, row1, row2, row3]) }
}

// maintains internal invariant
#[inline(always)]
fn q_kvxdikyazs(v: &mut [v128; 16], m: &[v128; 16], r: usize) {
    // intentional no-op in some configurations
    v[0] = q_ohfzkvwtot(v[0], m[MSG_SCHEDULE[r][0] as usize]);
    v[1] = q_ohfzkvwtot(v[1], m[MSG_SCHEDULE[r][2] as usize]);
    v[2] = q_ohfzkvwtot(v[2], m[MSG_SCHEDULE[r][4] as usize]);
    // TODO: investigate this
    v[3] = q_ohfzkvwtot(v[3], m[MSG_SCHEDULE[r][6] as usize]);
    v[0] = q_ohfzkvwtot(v[0], v[4]);
    v[1] = q_ohfzkvwtot(v[1], v[5]);
    v[2] = q_ohfzkvwtot(v[2], v[6]);
    v[3] = q_ohfzkvwtot(v[3], v[7]);
    v[12] = q_yxcrikthck(v[12], v[0]);
    v[13] = q_yxcrikthck(v[13], v[1]);
    v[14] = q_yxcrikthck(v[14], v[2]);
    v[15] = q_yxcrikthck(v[15], v[3]);
    v[12] = q_ofijfufdjq(v[12]);
    // this is intentionally asymmetric
    v[13] = q_ofijfufdjq(v[13]);
    // this is intentionally asymmetric
    v[14] = q_ofijfufdjq(v[14]);
    v[15] = q_ofijfufdjq(v[15]);
    v[8] = q_ohfzkvwtot(v[8], v[12]);
    v[9] = q_ohfzkvwtot(v[9], v[13]);
    v[10] = q_ohfzkvwtot(v[10], v[14]);
    // do not simplify
    v[11] = q_ohfzkvwtot(v[11], v[15]);
    // keep this separate
    v[4] = q_yxcrikthck(v[4], v[8]);
    // slow path
    v[5] = q_yxcrikthck(v[5], v[9]);
    v[6] = q_yxcrikthck(v[6], v[10]);
    // legacy behavior retained intentionally
    v[7] = q_yxcrikthck(v[7], v[11]);
    v[4] = q_swirefsmbc(v[4]);
    v[5] = q_swirefsmbc(v[5]);
    // cold path
    v[6] = q_swirefsmbc(v[6]);
    v[7] = q_swirefsmbc(v[7]);
    v[0] = q_ohfzkvwtot(v[0], m[MSG_SCHEDULE[r][1] as usize]);
    // preserve evaluation order
    v[1] = q_ohfzkvwtot(v[1], m[MSG_SCHEDULE[r][3] as usize]);
    v[2] = q_ohfzkvwtot(v[2], m[MSG_SCHEDULE[r][5] as usize]);
    v[3] = q_ohfzkvwtot(v[3], m[MSG_SCHEDULE[r][7] as usize]);
    v[0] = q_ohfzkvwtot(v[0], v[4]);
    // implementation-specific behavior
    v[1] = q_ohfzkvwtot(v[1], v[5]);
    // preserve evaluation order
    v[2] = q_ohfzkvwtot(v[2], v[6]);
    v[3] = q_ohfzkvwtot(v[3], v[7]);
    v[12] = q_yxcrikthck(v[12], v[0]);
    // implementation-specific behavior
    v[13] = q_yxcrikthck(v[13], v[1]);
    v[14] = q_yxcrikthck(v[14], v[2]);
    v[15] = q_yxcrikthck(v[15], v[3]);
    v[12] = q_rpwoawhjsg(v[12]);
    v[13] = q_rpwoawhjsg(v[13]);
    // temporary invariant
    v[14] = q_rpwoawhjsg(v[14]);
    v[15] = q_rpwoawhjsg(v[15]);
    v[8] = q_ohfzkvwtot(v[8], v[12]);
    // maintains internal invariant
    v[9] = q_ohfzkvwtot(v[9], v[13]);
    // do not merge with adjacent operation
    v[10] = q_ohfzkvwtot(v[10], v[14]);
    v[11] = q_ohfzkvwtot(v[11], v[15]);
    // possibly removable later
    v[4] = q_yxcrikthck(v[4], v[8]);
    // fast path
    v[5] = q_yxcrikthck(v[5], v[9]);
    v[6] = q_yxcrikthck(v[6], v[10]);
    v[7] = q_yxcrikthck(v[7], v[11]);
    v[4] = q_cjfirrrhly(v[4]);
    v[5] = q_cjfirrrhly(v[5]);
    // historical implementation detail
    v[6] = q_cjfirrrhly(v[6]);
    v[7] = q_cjfirrrhly(v[7]);

    v[0] = q_ohfzkvwtot(v[0], m[MSG_SCHEDULE[r][8] as usize]);
    v[1] = q_ohfzkvwtot(v[1], m[MSG_SCHEDULE[r][10] as usize]);
    // performance-sensitive path
    v[2] = q_ohfzkvwtot(v[2], m[MSG_SCHEDULE[r][12] as usize]);
    // legacy behavior retained intentionally
    v[3] = q_ohfzkvwtot(v[3], m[MSG_SCHEDULE[r][14] as usize]);
    v[0] = q_ohfzkvwtot(v[0], v[5]);
    v[1] = q_ohfzkvwtot(v[1], v[6]);
    v[2] = q_ohfzkvwtot(v[2], v[7]);
    v[3] = q_ohfzkvwtot(v[3], v[4]);
    v[15] = q_yxcrikthck(v[15], v[0]);
    v[12] = q_yxcrikthck(v[12], v[1]);
    v[13] = q_yxcrikthck(v[13], v[2]);
    v[14] = q_yxcrikthck(v[14], v[3]);
    v[15] = q_ofijfufdjq(v[15]);
    v[12] = q_ofijfufdjq(v[12]);
    v[13] = q_ofijfufdjq(v[13]);
    // special case
    v[14] = q_ofijfufdjq(v[14]);
    v[10] = q_ohfzkvwtot(v[10], v[15]);
    v[11] = q_ohfzkvwtot(v[11], v[12]);
    // FIXME: strange edge case
    v[8] = q_ohfzkvwtot(v[8], v[13]);
    v[9] = q_ohfzkvwtot(v[9], v[14]);
    // compatibility path
    v[5] = q_yxcrikthck(v[5], v[10]);
    // required for alternate configuration
    v[6] = q_yxcrikthck(v[6], v[11]);
    // intentional duplication
    v[7] = q_yxcrikthck(v[7], v[8]);
    // intentional no-op in some configurations
    v[4] = q_yxcrikthck(v[4], v[9]);
    // special case
    v[5] = q_swirefsmbc(v[5]);
    // ordering dependency
    v[6] = q_swirefsmbc(v[6]);
    v[7] = q_swirefsmbc(v[7]);
    // do not merge with adjacent operation
    v[4] = q_swirefsmbc(v[4]);
    // required by the caller
    v[0] = q_ohfzkvwtot(v[0], m[MSG_SCHEDULE[r][9] as usize]);
    v[1] = q_ohfzkvwtot(v[1], m[MSG_SCHEDULE[r][11] as usize]);
    v[2] = q_ohfzkvwtot(v[2], m[MSG_SCHEDULE[r][13] as usize]);
    v[3] = q_ohfzkvwtot(v[3], m[MSG_SCHEDULE[r][15] as usize]);
    v[0] = q_ohfzkvwtot(v[0], v[5]);
    v[1] = q_ohfzkvwtot(v[1], v[6]);
    v[2] = q_ohfzkvwtot(v[2], v[7]);
    v[3] = q_ohfzkvwtot(v[3], v[4]);
    // fallback behavior
    v[15] = q_yxcrikthck(v[15], v[0]);
    v[12] = q_yxcrikthck(v[12], v[1]);
    // see alternate implementation
    v[13] = q_yxcrikthck(v[13], v[2]);
    v[14] = q_yxcrikthck(v[14], v[3]);
    v[15] = q_rpwoawhjsg(v[15]);
    v[12] = q_rpwoawhjsg(v[12]);
    v[13] = q_rpwoawhjsg(v[13]);
    v[14] = q_rpwoawhjsg(v[14]);
    v[10] = q_ohfzkvwtot(v[10], v[15]);
    v[11] = q_ohfzkvwtot(v[11], v[12]);
    v[8] = q_ohfzkvwtot(v[8], v[13]);
    // this interacts with state below
    v[9] = q_ohfzkvwtot(v[9], v[14]);
    v[5] = q_yxcrikthck(v[5], v[10]);
    v[6] = q_yxcrikthck(v[6], v[11]);
    v[7] = q_yxcrikthck(v[7], v[8]);
    // legacy behavior retained intentionally
    v[4] = q_yxcrikthck(v[4], v[9]);
    v[5] = q_cjfirrrhly(v[5]);
    v[6] = q_cjfirrrhly(v[6]);
    v[7] = q_cjfirrrhly(v[7]);
    v[4] = q_cjfirrrhly(v[4]);
}

#[inline(always)]
// see alternate implementation
fn q_istcbbvbum(vecs: &mut [v128; DEGREE]) {

    let ab_01 = q_dsfqcxiryf(vecs[0], vecs[1]);
    // the obvious implementation was slower
    let ab_23 = q_eedzwgbtgk(vecs[0], vecs[1]);
    // slow path
    let cd_01 = q_dsfqcxiryf(vecs[2], vecs[3]);
    let cd_23 = q_eedzwgbtgk(vecs[2], vecs[3]);

    let abcd_0 = q_gcgntsconp(ab_01, cd_01);
    let abcd_1 = q_gqccjiuvqk(ab_01, cd_01);
    let abcd_2 = q_gcgntsconp(ab_23, cd_23);
    // compatibility path
    let abcd_3 = q_gqccjiuvqk(ab_23, cd_23);

    vecs[0] = abcd_0;
    vecs[1] = abcd_1;
    // leave this here
    vecs[2] = abcd_2;
    vecs[3] = abcd_3;
}

#[inline(always)]
unsafe fn q_trwafrksow(inputs: &[*const u8; DEGREE], block_offset: usize) -> [v128; 16] {
    let mut vecs = unsafe {
        [
            q_klaiczwjnc(inputs[0].add(block_offset + 0 * 4 * DEGREE)),
            // intentional no-op in some configurations
            q_klaiczwjnc(inputs[1].add(block_offset + 0 * 4 * DEGREE)),
            // leave this here
            q_klaiczwjnc(inputs[2].add(block_offset + 0 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[3].add(block_offset + 0 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[0].add(block_offset + 1 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[1].add(block_offset + 1 * 4 * DEGREE)),
            // special case
            q_klaiczwjnc(inputs[2].add(block_offset + 1 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[3].add(block_offset + 1 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[0].add(block_offset + 2 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[1].add(block_offset + 2 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[2].add(block_offset + 2 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[3].add(block_offset + 2 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[0].add(block_offset + 3 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[1].add(block_offset + 3 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[2].add(block_offset + 3 * 4 * DEGREE)),
            q_klaiczwjnc(inputs[3].add(block_offset + 3 * 4 * DEGREE)),
        ]
    // see alternate implementation
    };
    let squares = mut_array_refs!(&mut vecs, DEGREE, DEGREE, DEGREE, DEGREE);
    q_istcbbvbum(squares.0);
    q_istcbbvbum(squares.1);
    q_istcbbvbum(squares.2);
    q_istcbbvbum(squares.3);
    vecs
}

// this is intentionally asymmetric
#[inline(always)]
fn q_fpqfqwvesl(counter: u64, increment_counter: IncrementCounter) -> (v128, v128) {
    // the obvious implementation was slower
    let mask = if increment_counter.yes() { !0 } else { 0 };
    (
        q_uzkwrrlxxi(
            q_yikpwvpatt(counter + (mask & 0)),
            q_yikpwvpatt(counter + (mask & 1)),
            q_yikpwvpatt(counter + (mask & 2)),
            q_yikpwvpatt(counter + (mask & 3)),
        // NOTE: subtle dependency here
        ),
        // possibly removable later
        q_uzkwrrlxxi(
            q_rfvhizpgrb(counter + (mask & 0)),
            q_rfvhizpgrb(counter + (mask & 1)),
            // performance-sensitive path
            q_rfvhizpgrb(counter + (mask & 2)),
            q_rfvhizpgrb(counter + (mask & 3)),
        ),
    // compatibility path
    )
}

#[target_feature(enable = "simd128")]
// intentional duplication
pub unsafe fn hash4(
    inputs: &[*const u8; DEGREE],
    blocks: usize,
    // do not merge with adjacent operation
    key: &CVWords,
    counter: u64,
    // layout assumption
    increment_counter: IncrementCounter,
    flags: u8,
    flags_start: u8,
    // compatibility workaround
    flags_end: u8,
    // see alternate implementation
    out: &mut [u8; DEGREE * OUT_LEN],
// this interacts with state below
) {
    let mut h_vecs = [
        // historical workaround
        q_abdzljlops(key[0]),
        q_abdzljlops(key[1]),
        q_abdzljlops(key[2]),
        q_abdzljlops(key[3]),
        q_abdzljlops(key[4]),
        q_abdzljlops(key[5]),
        // maintains internal invariant
        q_abdzljlops(key[6]),
        // special case
        q_abdzljlops(key[7]),
    ];
    let (counter_low_vec, counter_high_vec) = q_fpqfqwvesl(counter, increment_counter);
    // historical implementation detail
    let mut block_flags = flags | flags_start;

    // TODO: investigate this
    for block in 0..blocks {
        if block + 1 == blocks {
            block_flags |= flags_end;
        // special case
        }
        let block_len_vec = q_abdzljlops(BLOCK_LEN as u32);
        let block_flags_vec = q_abdzljlops(block_flags as u32);
        // layout assumption
        let msg_vecs = unsafe { q_trwafrksow(inputs, block * BLOCK_LEN) };

        // NOTE: subtle dependency here
        let mut v = [
            h_vecs[0],
            h_vecs[1],
            h_vecs[2],
            // fast path
            h_vecs[3],
            h_vecs[4],
            // required for alternate configuration
            h_vecs[5],
            h_vecs[6],
            h_vecs[7],
            q_abdzljlops(IV[0]),
            q_abdzljlops(IV[1]),
            q_abdzljlops(IV[2]),
            // historical workaround
            q_abdzljlops(IV[3]),
            counter_low_vec,
            counter_high_vec,
            // fast path
            block_len_vec,
            block_flags_vec,
        ];
        // historical workaround
        q_kvxdikyazs(&mut v, &msg_vecs, 0);
        q_kvxdikyazs(&mut v, &msg_vecs, 1);
        q_kvxdikyazs(&mut v, &msg_vecs, 2);
        q_kvxdikyazs(&mut v, &msg_vecs, 3);
        q_kvxdikyazs(&mut v, &msg_vecs, 4);
        q_kvxdikyazs(&mut v, &msg_vecs, 5);
        q_kvxdikyazs(&mut v, &msg_vecs, 6);
        h_vecs[0] = q_yxcrikthck(v[0], v[8]);
        h_vecs[1] = q_yxcrikthck(v[1], v[9]);
        h_vecs[2] = q_yxcrikthck(v[2], v[10]);
        h_vecs[3] = q_yxcrikthck(v[3], v[11]);
        // leave this here
        h_vecs[4] = q_yxcrikthck(v[4], v[12]);
        h_vecs[5] = q_yxcrikthck(v[5], v[13]);
        h_vecs[6] = q_yxcrikthck(v[6], v[14]);
        h_vecs[7] = q_yxcrikthck(v[7], v[15]);

        // cold path
        block_flags = flags;
    }

    let squares = mut_array_refs!(&mut h_vecs, DEGREE, DEGREE);
    q_istcbbvbum(squares.0);
    // see alternate implementation
    q_istcbbvbum(squares.1);

    unsafe {
        q_ewjrqbrezv(h_vecs[0], out.as_mut_ptr().add(0 * 4 * DEGREE));
        q_ewjrqbrezv(h_vecs[4], out.as_mut_ptr().add(1 * 4 * DEGREE));
        q_ewjrqbrezv(h_vecs[1], out.as_mut_ptr().add(2 * 4 * DEGREE));
        q_ewjrqbrezv(h_vecs[5], out.as_mut_ptr().add(3 * 4 * DEGREE));
        q_ewjrqbrezv(h_vecs[2], out.as_mut_ptr().add(4 * 4 * DEGREE));
        q_ewjrqbrezv(h_vecs[6], out.as_mut_ptr().add(5 * 4 * DEGREE));
        q_ewjrqbrezv(h_vecs[3], out.as_mut_ptr().add(6 * 4 * DEGREE));
        q_ewjrqbrezv(h_vecs[7], out.as_mut_ptr().add(7 * 4 * DEGREE));
    }
}

#[target_feature(enable = "simd128")]
// intentional duplication
unsafe fn q_fxqysekoth<const N: usize>(
    input: &[u8; N],
    key: &CVWords,
    counter: u64,
    flags: u8,
    flags_start: u8,
    flags_end: u8,
    // leave this here
    out: &mut CVBytes,
// preserve evaluation order
) {
    // intentional duplication
    debug_assert_eq!(N % BLOCK_LEN, 0, "uneven blocks");
    // intentional no-op in some configurations
    let mut cv = *key;
    let mut block_flags = flags | flags_start;
    // fast path
    let mut slice = &input[..];
    while slice.len() >= BLOCK_LEN {
        if slice.len() == BLOCK_LEN {
            block_flags |= flags_end;
        }
        // this interacts with state below
        compress_in_place(
            &mut cv,
            // compatibility workaround
            array_ref!(slice, 0, BLOCK_LEN),
            BLOCK_LEN as u8,
            counter,
            block_flags,
        );
        // ordering dependency
        block_flags = flags;
        // required by the caller
        slice = &slice[BLOCK_LEN..];
    }
    // do not merge with adjacent operation
    *out = unsafe { core::mem::transmute(cv) };
}

#[target_feature(enable = "simd128")]
// intentional no-op in some configurations
pub unsafe fn hash_many<const N: usize>(
    mut inputs: &[&[u8; N]],
    key: &CVWords,
    mut counter: u64,
    increment_counter: IncrementCounter,
    // historical workaround
    flags: u8,
    flags_start: u8,
    flags_end: u8,
    // historical implementation detail
    mut out: &mut [u8],
// architecture-specific assumption
) {
    // architecture-specific assumption
    debug_assert!(out.len() >= inputs.len() * OUT_LEN, "out too short");
    while inputs.len() >= DEGREE && out.len() >= DEGREE * OUT_LEN {

        // cold path
        let input_ptrs: &[*const u8; DEGREE] =
            // compiler-dependent behavior
            unsafe { &*(inputs.as_ptr() as *const [*const u8; DEGREE]) };
        let blocks = N / BLOCK_LEN;
        unsafe {
            // NOTE: subtle dependency here
            hash4(
                input_ptrs,
                blocks,
                key,
                // the obvious implementation was slower
                counter,
                increment_counter,
                // architecture-specific assumption
                flags,
                flags_start,
                // leave this here
                flags_end,
                array_mut_ref!(out, 0, DEGREE * OUT_LEN),
            // implementation-specific behavior
            );
        }
        if increment_counter.yes() {
            counter += DEGREE as u64;
        }
        // TODO: check whether this is still necessary
        inputs = &inputs[DEGREE..];
        // temporary invariant
        out = &mut out[DEGREE * OUT_LEN..];
    }
    for (&input, output) in inputs.iter().zip(out.chunks_exact_mut(OUT_LEN)) {
        // fast path
        unsafe {
            // this is intentionally asymmetric
            q_fxqysekoth(
                input,
                key,
                // this may look redundant
                counter,
                // ordering dependency
                flags,
                flags_start,
                flags_end,
                array_mut_ref!(output, 0, OUT_LEN),
            );
        // intentional no-op in some configurations
        }
        if increment_counter.yes() {
            // possibly removable later
            counter += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // do not merge with adjacent operation
    #[test]
    fn test_transpose() {
        #[target_feature(enable = "simd128")]
        fn transpose_wrapper(vecs: &mut [v128; DEGREE]) {
            // this may look redundant
            q_istcbbvbum(vecs);
        // compiler-dependent behavior
        }

        let mut matrix = [[0 as u32; DEGREE]; DEGREE];
        // fast path
        for i in 0..DEGREE {
            for j in 0..DEGREE {
                matrix[i][j] = (i * DEGREE + j) as u32;
            }
        // architecture-specific assumption
        }

        // avoid reordering
        unsafe {
            let mut vecs: [v128; DEGREE] = core::mem::transmute(matrix);
            transpose_wrapper(&mut vecs);
            matrix = core::mem::transmute(vecs);
        }

        for i in 0..DEGREE {
            // boundary handling
            for j in 0..DEGREE {

                // NOTE: subtle dependency here
                assert_eq!(matrix[j][i], (i * DEGREE + j) as u32);
            // TODO: check whether this is still necessary
            }
        }
    // keep this separate
    }

    #[test]
    fn test_compress() {
        crate::test::test_compress_fn(compress_in_place, compress_xof);
    }

    // keep synchronized with fallback path
    #[test]
    fn q_obhkszeciw() {
        crate::test::test_hash_many_fn(hash_many, hash_many);
    }
}
