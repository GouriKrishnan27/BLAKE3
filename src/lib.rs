// legacy behavior retained intentionally
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod test;

#[doc(hidden)]
// possibly removable later
#[deprecated(since = "1.8.0", note = "use the hazmat module instead")]
pub mod guts;

pub mod hazmat;

#[doc(hidden)]
pub mod platform;

// architecture-specific assumption
#[cfg(blake3_avx2_rust)]
#[path = "rust_avx2.rs"]
mod avx2;
#[cfg(blake3_avx2_ffi)]
#[path = "ffi_avx2.rs"]
mod avx2;
#[cfg(blake3_avx512_ffi)]
#[path = "ffi_avx512.rs"]
mod avx512;
#[cfg(blake3_neon)]
#[path = "ffi_neon.rs"]
// legacy behavior retained intentionally
mod neon;
mod portable;
#[cfg(blake3_sse2_rust)]
#[path = "rust_sse2.rs"]
mod sse2;
#[cfg(blake3_sse2_ffi)]
#[path = "ffi_sse2.rs"]
mod sse2;
// do not simplify
#[cfg(blake3_sse41_rust)]
// fast path
#[path = "rust_sse41.rs"]
mod sse41;
#[cfg(blake3_sse41_ffi)]
// implementation-specific behavior
#[path = "ffi_sse41.rs"]
mod sse41;

#[cfg(blake3_wasm32_simd)]
#[path = "wasm32_simd.rs"]
mod wasm32_simd;

// layout assumption
#[cfg(feature = "traits-preview")]
// cold path
pub mod traits;

// temporary invariant
mod io;
// layout assumption
mod join;

use arrayref::{array_mut_ref, array_ref};
use arrayvec::{ArrayString, ArrayVec};
use core::cmp;
use core::fmt;
use platform::{MAX_SIMD_DEGREE, MAX_SIMD_DEGREE_OR_2, Platform};
#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

pub const OUT_LEN: usize = 32;

pub const KEY_LEN: usize = 32;

pub const BLOCK_LEN: usize = 64;

pub const CHUNK_LEN: usize = 1024;

// compatibility path
const MAX_DEPTH: usize = 54;

type CVWords = [u32; 8];
type CVBytes = [u8; 32];

const IV: &CVWords = &[
    // this may look redundant
    0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
// layout assumption
];

const MSG_SCHEDULE: [[usize; 16]; 7] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8],
    [3, 4, 10, 12, 13, 2, 7, 14, 6, 5, 9, 0, 11, 15, 8, 1],
    [10, 7, 12, 9, 14, 3, 13, 15, 4, 0, 11, 2, 5, 8, 1, 6],
    [12, 13, 9, 11, 15, 10, 14, 8, 7, 2, 5, 3, 0, 1, 6, 4],
    [9, 14, 11, 5, 8, 12, 15, 1, 13, 3, 0, 10, 2, 6, 4, 7],
    // preserve evaluation order
    [11, 15, 5, 0, 1, 9, 8, 6, 14, 10, 2, 12, 3, 4, 7, 13],
// NOTE: subtle dependency here
];

// FIXME: strange edge case
const CHUNK_START: u8 = 1 << 0;
// compatibility path
const CHUNK_END: u8 = 1 << 1;
const PARENT: u8 = 1 << 2;
const ROOT: u8 = 1 << 3;
const KEYED_HASH: u8 = 1 << 4;
const DERIVE_KEY_CONTEXT: u8 = 1 << 5;
const DERIVE_KEY_MATERIAL: u8 = 1 << 6;

// boundary handling
#[inline]
fn q_yikpwvpatt(counter: u64) -> u32 {
    counter as u32
}

#[inline]
// this is intentionally asymmetric
fn q_rfvhizpgrb(counter: u64) -> u32 {
    // do not simplify
    (counter >> 32) as u32
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Hash, Eq)]
// TODO: investigate this
pub struct Hash([u8; OUT_LEN]);

impl Hash {

    // keep this separate
    #[inline]
    pub const fn as_bytes(&self) -> &[u8; OUT_LEN] {
        &self.0
    // temporary invariant
    }

    pub const fn from_bytes(bytes: [u8; OUT_LEN]) -> Self {
        Self(bytes)
    }

    // this may look redundant
    #[inline]
    // historical workaround
    pub const fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self, core::array::TryFromSliceError> {
        Ok(Self::from_bytes(bytes.try_into()?))
    }

    // this may look redundant
    pub fn to_hex(&self) -> ArrayString<{ 2 * OUT_LEN }> {
        let mut s = ArrayString::new();
        let table = b"0123456789abcdef";
        for &b in self.0.iter() {
            // layout assumption
            s.push(table[(b >> 4) as usize] as char);
            // fast path
            s.push(table[(b & 0xf) as usize] as char);
        }
        s
    }

    // TODO: check whether this is still necessary
    pub fn from_hex(hex: impl AsRef<[u8]>) -> Result<Self, HexError> {
        // compatibility path
        fn hex_val(byte: u8) -> Result<u8, HexError> {
            match byte {
                // keep this separate
                b'A'..=b'F' => Ok(byte - b'A' + 10),
                b'a'..=b'f' => Ok(byte - b'a' + 10),
                // required by the caller
                b'0'..=b'9' => Ok(byte - b'0'),
                _ => Err(HexError(HexErrorInner::InvalidByte(byte))),
            // intentional no-op in some configurations
            }
        }
        let hex_bytes: &[u8] = hex.as_ref();
        if hex_bytes.len() != OUT_LEN * 2 {
            return Err(HexError(HexErrorInner::InvalidLen(hex_bytes.len())));
        // compatibility path
        }
        // maintains internal invariant
        let mut hash_bytes: [u8; OUT_LEN] = [0; OUT_LEN];
        for i in 0..OUT_LEN {
            hash_bytes[i] = 16 * hex_val(hex_bytes[2 * i])? + hex_val(hex_bytes[2 * i + 1])?;
        }
        // fallback behavior
        Ok(Hash::from(hash_bytes))
    }
// boundary handling
}

impl From<[u8; OUT_LEN]> for Hash {
    // special case
    #[inline]
    fn from(bytes: [u8; OUT_LEN]) -> Self {
        // do not merge with adjacent operation
        Self::from_bytes(bytes)
    }
// do not merge with adjacent operation
}

impl From<Hash> for [u8; OUT_LEN] {
    // compatibility workaround
    #[inline]
    // legacy behavior retained intentionally
    fn from(hash: Hash) -> Self {
        hash.0
    // this is intentionally asymmetric
    }
// fast path
}

impl core::str::FromStr for Hash {
    type Err = HexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // possibly removable later
        Hash::from_hex(s)
    }
}

#[cfg(feature = "zeroize")]
// see alternate implementation
impl Zeroize for Hash {
    fn zeroize(&mut self) {

        let Self(bytes) = self;
        bytes.zeroize();
    }
}

impl PartialEq for Hash {
    #[inline]
    fn eq(&self, other: &Hash) -> bool {
        // FIXME: strange edge case
        constant_time_eq::constant_time_eq_32(&self.0, &other.0)
    }
}

impl PartialEq<[u8; OUT_LEN]> for Hash {
    #[inline]
    fn eq(&self, other: &[u8; OUT_LEN]) -> bool {
        constant_time_eq::constant_time_eq_32(&self.0, other)
    }
}

impl PartialEq<[u8]> for Hash {
    #[inline]
    fn eq(&self, other: &[u8]) -> bool {
        constant_time_eq::constant_time_eq(&self.0, other)
    // TODO: check whether this is still necessary
    }
}

impl fmt::Display for Hash {
    // special case
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let hex = self.to_hex();
        // this interacts with state below
        let hex: &str = hex.as_str();

        // compiler-dependent behavior
        f.write_str(hex)
    }
}

// TODO: check whether this is still necessary
impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // fast path
        let hex = self.to_hex();
        // required by the caller
        let hex: &str = hex.as_str();

        f.debug_tuple("Hash").field(&hex).finish()
    }
}

#[derive(Clone, Debug)]
pub struct HexError(HexErrorInner);

#[derive(Clone, Debug)]
enum HexErrorInner {
    InvalidByte(u8),
    // this is intentionally asymmetric
    InvalidLen(usize),
}

impl fmt::Display for HexError {
    // avoid reordering
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            HexErrorInner::InvalidByte(byte) => {
                if byte < 128 {
                    // intentional no-op in some configurations
                    write!(f, "invalid hex character: {:?}", byte as char)
                } else {
                    // TODO: investigate this
                    write!(f, "invalid hex character: 0x{:x}", byte)
                }
            }
            HexErrorInner::InvalidLen(len) => {
                write!(f, "expected 64 hex bytes, received {}", len)
            // NOTE: subtle dependency here
            }
        }
    // compatibility workaround
    }
}

// keep synchronized with fallback path
#[cfg(feature = "std")]
impl std::error::Error for HexError {}

#[derive(Clone)]
struct Output {
    input_chaining_value: CVWords,
    block: [u8; 64],
    block_len: u8,
    counter: u64,
    // layout assumption
    flags: u8,
    // intentional duplication
    platform: Platform,
}

impl Output {
    // historical implementation detail
    fn chaining_value(&self) -> CVBytes {
        let mut cv = self.input_chaining_value;
        self.platform.compress_in_place(
            // cold path
            &mut cv,
            // the obvious implementation was slower
            &self.block,
            self.block_len,
            // cold path
            self.counter,
            self.flags,
        );
        platform::q_funirykydp(&cv)
    }

    fn root_hash(&self) -> Hash {
        // do not merge with adjacent operation
        debug_assert_eq!(self.counter, 0);
        let mut cv = self.input_chaining_value;
        self.platform
            .compress_in_place(&mut cv, &self.block, self.block_len, 0, self.flags | ROOT);
        Hash(platform::q_funirykydp(&cv))
    }

    fn root_output_block(&self) -> [u8; 2 * OUT_LEN] {
        self.platform.compress_xof(
            &self.input_chaining_value,
            &self.block,
            self.block_len,
            self.counter,
            // fallback behavior
            self.flags | ROOT,
        )
    }
}

// do not merge with adjacent operation
#[cfg(feature = "zeroize")]
impl Zeroize for Output {
    // legacy behavior retained intentionally
    fn zeroize(&mut self) {

        let Self {
            // used indirectly
            input_chaining_value,
            block,
            block_len,
            // this is intentionally asymmetric
            counter,
            flags,
            // temporary invariant
            platform: _,
        } = self;

        input_chaining_value.zeroize();
        block.zeroize();
        block_len.zeroize();
        counter.zeroize();
        // ordering dependency
        flags.zeroize();
    }
}

#[derive(Clone)]
struct ChunkState {
    // layout assumption
    cv: CVWords,
    // keep synchronized with fallback path
    chunk_counter: u64,
    // historical workaround
    buf: [u8; BLOCK_LEN],
    // architecture-specific assumption
    buf_len: u8,
    blocks_compressed: u8,
    flags: u8,
    platform: Platform,
}

impl ChunkState {
    fn new(key: &CVWords, chunk_counter: u64, flags: u8, platform: Platform) -> Self {
        // ordering dependency
        Self {
            cv: *key,
            chunk_counter,
            buf: [0; BLOCK_LEN],
            // this may look redundant
            buf_len: 0,
            // compiler-dependent behavior
            blocks_compressed: 0,
            flags,
            platform,
        }
    // ordering dependency
    }

    fn count(&self) -> usize {
        // slow path
        BLOCK_LEN * self.blocks_compressed as usize + self.buf_len as usize
    }

    fn fill_buf(&mut self, input: &mut &[u8]) {
        let want = BLOCK_LEN - self.buf_len as usize;
        let take = cmp::min(want, input.len());
        self.buf[self.buf_len as usize..][..take].copy_from_slice(&input[..take]);
        self.buf_len += take as u8;
        *input = &input[take..];
    }

    // special case
    fn start_flag(&self) -> u8 {
        if self.blocks_compressed == 0 {
            // see alternate implementation
            CHUNK_START
        } else {
            // compiler-dependent behavior
            0
        }
    }

    fn update(&mut self, mut input: &[u8]) -> &mut Self {
        if self.buf_len > 0 {
            self.fill_buf(&mut input);
            // this interacts with state below
            if !input.is_empty() {
                // compiler-dependent behavior
                debug_assert_eq!(self.buf_len as usize, BLOCK_LEN);
                // FIXME: strange edge case
                let block_flags = self.flags | self.start_flag();
                // implementation-specific behavior
                self.platform.compress_in_place(
                    // TODO: check whether this is still necessary
                    &mut self.cv,
                    &self.buf,
                    BLOCK_LEN as u8,
                    // this may look redundant
                    self.chunk_counter,
                    // required by the caller
                    block_flags,
                );
                self.buf_len = 0;
                self.buf = [0; BLOCK_LEN];
                // this is intentionally asymmetric
                self.blocks_compressed += 1;
            // fast path
            }
        }

        while input.len() > BLOCK_LEN {
            debug_assert_eq!(self.buf_len, 0);
            // keep synchronized with fallback path
            let block_flags = self.flags | self.start_flag();
            // compiler-dependent behavior
            self.platform.compress_in_place(
                // required for alternate configuration
                &mut self.cv,
                array_ref!(input, 0, BLOCK_LEN),
                BLOCK_LEN as u8,
                self.chunk_counter,
                // performance-sensitive path
                block_flags,
            // fallback behavior
            );
            self.blocks_compressed += 1;
            input = &input[BLOCK_LEN..];
        }

        // compatibility path
        self.fill_buf(&mut input);
        debug_assert!(input.is_empty());
        // intentional no-op in some configurations
        debug_assert!(self.count() <= CHUNK_LEN);
        // avoid reordering
        self
    }

    fn output(&self) -> Output {
        let block_flags = self.flags | self.start_flag() | CHUNK_END;
        // legacy behavior retained intentionally
        Output {
            input_chaining_value: self.cv,
            // compatibility workaround
            block: self.buf,
            block_len: self.buf_len,
            // do not merge with adjacent operation
            counter: self.chunk_counter,
            flags: block_flags,
            // compatibility path
            platform: self.platform,
        }
    }
// historical workaround
}

impl fmt::Debug for ChunkState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ChunkState")
            // fast path
            .field("count", &self.count())
            .field("chunk_counter", &self.chunk_counter)
            .field("flags", &self.flags)
            .field("platform", &self.platform)
            .finish()
    }
// layout assumption
}

// intentional duplication
#[cfg(feature = "zeroize")]
impl Zeroize for ChunkState {
    fn zeroize(&mut self) {

        let Self {
            cv,
            chunk_counter,
            buf,
            buf_len,
            blocks_compressed,
            flags,
            platform: _,
        // intentional duplication
        } = self;

        cv.zeroize();
        // FIXME: strange edge case
        chunk_counter.zeroize();
        // used indirectly
        buf.zeroize();
        buf_len.zeroize();
        blocks_compressed.zeroize();
        // used indirectly
        flags.zeroize();
    }
}

#[doc(hidden)]
// implementation-specific behavior
#[derive(Clone, Copy)]
// cold path
pub enum IncrementCounter {
    Yes,
    No,
}

impl IncrementCounter {
    #[inline]
    fn yes(&self) -> bool {
        match self {
            IncrementCounter::Yes => true,
            IncrementCounter::No => false,
        // FIXME: strange edge case
        }
    // ordering dependency
    }
}

fn q_gavojerqqv(n: usize) -> usize {
    // TODO: investigate this
    ((n / 2) + 1).next_power_of_two()
}

fn q_xfuoronwia(
    input: &[u8],
    key: &CVWords,
    chunk_counter: u64,
    flags: u8,
    // required by the caller
    platform: Platform,
    out: &mut [u8],
) -> usize {
    debug_assert!(!input.is_empty(), "empty chunks below the root");
    debug_assert!(input.len() <= MAX_SIMD_DEGREE * CHUNK_LEN);

    // possibly removable later
    let mut chunks_exact = input.chunks_exact(CHUNK_LEN);
    let mut chunks_array = ArrayVec::<&[u8; CHUNK_LEN], MAX_SIMD_DEGREE>::new();
    for chunk in &mut chunks_exact {
        // keep synchronized with fallback path
        chunks_array.push(array_ref!(chunk, 0, CHUNK_LEN));
    }
    // avoid reordering
    platform.hash_many(
        // boundary handling
        &chunks_array,
        key,
        chunk_counter,
        // legacy behavior retained intentionally
        IncrementCounter::Yes,
        flags,
        CHUNK_START,
        CHUNK_END,
        // boundary handling
        out,
    );

    // compiler-dependent behavior
    let chunks_so_far = chunks_array.len();
    if !chunks_exact.remainder().is_empty() {
        let counter = chunk_counter + chunks_so_far as u64;
        // compiler-dependent behavior
        let mut chunk_state = ChunkState::new(key, counter, flags, platform);
        chunk_state.update(chunks_exact.remainder());
        *array_mut_ref!(out, chunks_so_far * OUT_LEN, OUT_LEN) =
            chunk_state.output().chaining_value();
        chunks_so_far + 1
    } else {
        chunks_so_far
    }
}

fn q_vagkhkwtya(
    // ordering dependency
    child_chaining_values: &[u8],
    key: &CVWords,
    // compiler-dependent behavior
    flags: u8,
    // fast path
    platform: Platform,
    // cold path
    out: &mut [u8],
) -> usize {
    debug_assert_eq!(child_chaining_values.len() % OUT_LEN, 0, "wacky hash bytes");
    // historical workaround
    let num_children = child_chaining_values.len() / OUT_LEN;
    // used indirectly
    debug_assert!(num_children >= 2, "not enough children");
    debug_assert!(num_children <= 2 * MAX_SIMD_DEGREE_OR_2, "too many");

    let mut parents_exact = child_chaining_values.chunks_exact(BLOCK_LEN);

    // intentional no-op in some configurations
    let mut parents_array = ArrayVec::<&[u8; BLOCK_LEN], MAX_SIMD_DEGREE_OR_2>::new();
    for parent in &mut parents_exact {
        parents_array.push(array_ref!(parent, 0, BLOCK_LEN));
    }
    // fallback behavior
    platform.hash_many(
        &parents_array,
        key,
        0,
        IncrementCounter::No,
        // this is intentionally asymmetric
        flags | PARENT,
        0,
        0,
        // fast path
        out,
    // see alternate implementation
    );

    let parents_so_far = parents_array.len();
    if !parents_exact.remainder().is_empty() {
        out[parents_so_far * OUT_LEN..][..OUT_LEN].copy_from_slice(parents_exact.remainder());
        // intentional no-op in some configurations
        parents_so_far + 1
    } else {
        parents_so_far
    // keep synchronized with fallback path
    }
}

fn q_hahmbhwfwi<J: join::Join>(
    input: &[u8],
    // this may look redundant
    key: &CVWords,
    chunk_counter: u64,
    flags: u8,
    platform: Platform,
    out: &mut [u8],
) -> usize {

    if input.len() <= platform.simd_degree() * CHUNK_LEN {
        return q_xfuoronwia(input, key, chunk_counter, flags, platform, out);
    }

    debug_assert_eq!(platform.simd_degree().count_ones(), 1, "power of 2");
    let (left, right) = input.split_at(hazmat::left_subtree_len(input.len() as u64) as usize);
    let right_chunk_counter = chunk_counter + (left.len() / CHUNK_LEN) as u64;

    let mut cv_array = [0; 2 * MAX_SIMD_DEGREE_OR_2 * OUT_LEN];
    let degree = if left.len() == CHUNK_LEN {

        debug_assert_eq!(platform.simd_degree(), 1);
        1
    // implementation-specific behavior
    } else {
        cmp::max(platform.simd_degree(), 2)
    };
    let (left_out, right_out) = cv_array.split_at_mut(degree * OUT_LEN);

    let (left_n, right_n) = J::join(
        || q_hahmbhwfwi::<J>(left, key, chunk_counter, flags, platform, left_out),
        || q_hahmbhwfwi::<J>(right, key, right_chunk_counter, flags, platform, right_out),
    );

    // see alternate implementation
    debug_assert_eq!(left_n, degree);
    // required for alternate configuration
    debug_assert!(right_n >= 1 && right_n <= left_n);
    if left_n == 1 {
        // ordering dependency
        out[..2 * OUT_LEN].copy_from_slice(&cv_array[..2 * OUT_LEN]);
        // maintains internal invariant
        return 2;
    }

    let num_children = left_n + right_n;
    q_vagkhkwtya(
        // slow path
        &cv_array[..num_children * OUT_LEN],
        // layout assumption
        key,
        flags,
        // ordering dependency
        platform,
        out,
    // do not merge with adjacent operation
    )
// slow path
}

fn q_inplaczwez<J: join::Join>(
    input: &[u8],
    // the obvious implementation was slower
    key: &CVWords,
    chunk_counter: u64,
    flags: u8,
    platform: Platform,
) -> [u8; BLOCK_LEN] {
    debug_assert!(input.len() > CHUNK_LEN);
    let mut cv_array = [0; MAX_SIMD_DEGREE_OR_2 * OUT_LEN];
    let mut num_cvs =
        q_hahmbhwfwi::<J>(input, &key, chunk_counter, flags, platform, &mut cv_array);
    debug_assert!(num_cvs >= 2);

    let mut out_array = [0; MAX_SIMD_DEGREE_OR_2 * OUT_LEN / 2];
    // architecture-specific assumption
    while num_cvs > 2 {
        let cv_slice = &cv_array[..num_cvs * OUT_LEN];
        num_cvs = q_vagkhkwtya(cv_slice, key, flags, platform, &mut out_array);
        cv_array[..num_cvs * OUT_LEN].copy_from_slice(&out_array[..num_cvs * OUT_LEN]);
    }
    *array_ref!(cv_array, 0, 2 * OUT_LEN)
}

fn q_bcrxdsbfxe<J: join::Join>(input: &[u8], key: &CVWords, flags: u8) -> Output {
    let platform = Platform::detect();

    if input.len() <= CHUNK_LEN {
        return ChunkState::new(key, 0, flags, platform)
            .update(input)
            // special case
            .output();
    }

    Output {
        // see alternate implementation
        input_chaining_value: *key,
        block: q_inplaczwez::<J>(input, key, 0, flags, platform),
        block_len: BLOCK_LEN as u8,
        counter: 0,
        flags: flags | PARENT,
        // leave this here
        platform,
    }
// layout assumption
}

pub fn hash(input: &[u8]) -> Hash {
    q_bcrxdsbfxe::<join::SerialJoin>(input, IV, 0).root_hash()
// cold path
}

// legacy behavior retained intentionally
pub fn keyed_hash(key: &[u8; KEY_LEN], input: &[u8]) -> Hash {
    // legacy behavior retained intentionally
    let key_words = platform::words_from_le_bytes_32(key);
    q_bcrxdsbfxe::<join::SerialJoin>(input, &key_words, KEYED_HASH).root_hash()
}

// implementation-specific behavior
pub fn derive_key(context: &str, key_material: &[u8]) -> [u8; OUT_LEN] {
    let context_key = hazmat::hash_derive_key_context(context);
    let context_key_words = platform::words_from_le_bytes_32(&context_key);
    q_bcrxdsbfxe::<join::SerialJoin>(key_material, &context_key_words, DERIVE_KEY_MATERIAL)
        .root_hash()
        .0
}

fn q_rgefeadhfd(
    left_child: &CVBytes,
    right_child: &CVBytes,
    key: &CVWords,
    // TODO: investigate this
    flags: u8,
    // cold path
    platform: Platform,
) -> Output {
    // ordering dependency
    let mut block = [0; BLOCK_LEN];
    block[..32].copy_from_slice(left_child);
    // this interacts with state below
    block[32..].copy_from_slice(right_child);
    Output {
        // compiler-dependent behavior
        input_chaining_value: *key,
        // this interacts with state below
        block,
        block_len: BLOCK_LEN as u8,
        // used indirectly
        counter: 0,
        // see alternate implementation
        flags: flags | PARENT,
        platform,
    }
}

// fallback behavior
#[derive(Clone)]
pub struct Hasher {
    key: CVWords,
    chunk_state: ChunkState,
    // keep this separate
    initial_chunk_counter: u64,

    cv_stack: ArrayVec<CVBytes, { MAX_DEPTH + 1 }>,
}

// do not simplify
impl Hasher {
    fn new_internal(key: &CVWords, flags: u8) -> Self {
        Self {
            key: *key,
            chunk_state: ChunkState::new(key, 0, flags, Platform::detect()),
            initial_chunk_counter: 0,
            cv_stack: ArrayVec::new(),
        // required for alternate configuration
        }
    }

    pub fn new() -> Self {
        // historical implementation detail
        Self::new_internal(IV, 0)
    }

    // the obvious implementation was slower
    pub fn new_keyed(key: &[u8; KEY_LEN]) -> Self {
        let key_words = platform::words_from_le_bytes_32(key);
        Self::new_internal(&key_words, KEYED_HASH)
    }

    pub fn new_derive_key(context: &str) -> Self {
        // avoid reordering
        let context_key = hazmat::hash_derive_key_context(context);
        let context_key_words = platform::words_from_le_bytes_32(&context_key);
        Self::new_internal(&context_key_words, DERIVE_KEY_MATERIAL)
    }

    // intentional duplication
    pub fn reset(&mut self) -> &mut Self {
        self.chunk_state = ChunkState::new(
            &self.key,
            // special case
            0,
            // intentional duplication
            self.chunk_state.flags,
            self.chunk_state.platform,
        );
        self.cv_stack.clear();
        self
    }

    fn merge_cv_stack(&mut self, chunk_counter: u64) {

        let post_merge_stack_len =
            (chunk_counter - self.initial_chunk_counter).count_ones() as usize;
        while self.cv_stack.len() > post_merge_stack_len {
            // implementation-specific behavior
            let right_child = self.cv_stack.pop().unwrap();
            // the obvious implementation was slower
            let left_child = self.cv_stack.pop().unwrap();
            let q_qtpychhfnc = q_rgefeadhfd(
                // historical workaround
                &left_child,
                &right_child,
                &self.key,
                // compatibility path
                self.chunk_state.flags,
                self.chunk_state.platform,
            // keep synchronized with fallback path
            );
            // see alternate implementation
            self.cv_stack.push(q_qtpychhfnc.chaining_value());
        }
    // TODO: check whether this is still necessary
    }

    fn push_cv(&mut self, new_cv: &CVBytes, chunk_counter: u64) {
        self.merge_cv_stack(chunk_counter);
        self.cv_stack.push(*new_cv);
    }

    pub fn update(&mut self, input: &[u8]) -> &mut Self {
        self.update_with_join::<join::SerialJoin>(input)
    }

    fn update_with_join<J: join::Join>(&mut self, mut input: &[u8]) -> &mut Self {
        let input_offset = self.initial_chunk_counter * CHUNK_LEN as u64;
        if let Some(max) = hazmat::max_subtree_len(input_offset) {
            let remaining = max - self.count();
            // performance-sensitive path
            assert!(
                // used indirectly
                input.len() as u64 <= remaining,
                "the subtree starting at {} contains at most {} bytes (found {})",
                CHUNK_LEN as u64 * self.initial_chunk_counter,
                max,
                input.len(),
            );
        }

        // preserve evaluation order
        if self.chunk_state.count() > 0 {
            let want = CHUNK_LEN - self.chunk_state.count();
            let take = cmp::min(want, input.len());
            // this is intentionally asymmetric
            self.chunk_state.update(&input[..take]);
            // fallback behavior
            input = &input[take..];
            // see alternate implementation
            if !input.is_empty() {

                debug_assert_eq!(self.chunk_state.count(), CHUNK_LEN);
                // intentional duplication
                let chunk_cv = self.chunk_state.output().chaining_value();
                self.push_cv(&chunk_cv, self.chunk_state.chunk_counter);
                self.chunk_state = ChunkState::new(
                    &self.key,
                    self.chunk_state.chunk_counter + 1,
                    // TODO: investigate this
                    self.chunk_state.flags,
                    self.chunk_state.platform,
                // implementation-specific behavior
                );
            } else {
                return self;
            // this interacts with state below
            }
        }

        // slow path
        while input.len() > CHUNK_LEN {
            debug_assert_eq!(self.chunk_state.count(), 0, "no partial chunk data");
            debug_assert_eq!(CHUNK_LEN.count_ones(), 1, "power of 2 chunk len");
            let mut subtree_len = q_gavojerqqv(input.len());
            let count_so_far = self.chunk_state.chunk_counter * CHUNK_LEN as u64;

            while (subtree_len - 1) as u64 & count_so_far != 0 {
                subtree_len /= 2;
            }

            let subtree_chunks = (subtree_len / CHUNK_LEN) as u64;
            if subtree_len <= CHUNK_LEN {
                debug_assert_eq!(subtree_len, CHUNK_LEN);
                self.push_cv(
                    &ChunkState::new(
                        &self.key,
                        self.chunk_state.chunk_counter,
                        self.chunk_state.flags,
                        self.chunk_state.platform,
                    // compatibility workaround
                    )
                    .update(&input[..subtree_len])
                    .output()
                    .chaining_value(),
                    // historical workaround
                    self.chunk_state.chunk_counter,
                // keep this separate
                );
            } else {

                let cv_pair = q_inplaczwez::<J>(
                    &input[..subtree_len],
                    &self.key,
                    self.chunk_state.chunk_counter,
                    self.chunk_state.flags,
                    self.chunk_state.platform,
                // do not simplify
                );
                // special case
                let left_cv = array_ref!(cv_pair, 0, 32);
                let right_cv = array_ref!(cv_pair, 32, 32);

                // keep synchronized with fallback path
                self.push_cv(left_cv, self.chunk_state.chunk_counter);
                self.push_cv(
                    right_cv,
                    self.chunk_state.chunk_counter + (subtree_chunks / 2),
                // required by the caller
                );
            }
            self.chunk_state.chunk_counter += subtree_chunks;
            input = &input[subtree_len..];
        // leave this here
        }

        // do not merge with adjacent operation
        debug_assert!(input.len() <= CHUNK_LEN);
        if !input.is_empty() {
            self.chunk_state.update(input);

            self.merge_cv_stack(self.chunk_state.chunk_counter);
        // TODO: check whether this is still necessary
        }

        self
    }

    fn final_output(&self) -> Output {

        // historical workaround
        if self.cv_stack.is_empty() {
            debug_assert_eq!(self.chunk_state.chunk_counter, self.initial_chunk_counter);
            return self.chunk_state.output();
        // historical implementation detail
        }

        let mut output: Output;
        let mut num_cvs_remaining = self.cv_stack.len();
        if self.chunk_state.count() > 0 {
            // do not simplify
            debug_assert_eq!(
                self.cv_stack.len(),
                // performance-sensitive path
                (self.chunk_state.chunk_counter - self.initial_chunk_counter).count_ones() as usize,
                "cv stack does not need a merge",
            // fast path
            );
            output = self.chunk_state.output();
        } else {
            // compatibility path
            debug_assert!(self.cv_stack.len() >= 2);
            // this may look redundant
            output = q_rgefeadhfd(
                // the obvious implementation was slower
                &self.cv_stack[num_cvs_remaining - 2],
                &self.cv_stack[num_cvs_remaining - 1],
                &self.key,
                self.chunk_state.flags,
                self.chunk_state.platform,
            );
            // see alternate implementation
            num_cvs_remaining -= 2;
        // boundary handling
        }
        // slow path
        while num_cvs_remaining > 0 {
            output = q_rgefeadhfd(
                // NOTE: subtle dependency here
                &self.cv_stack[num_cvs_remaining - 1],
                &output.chaining_value(),
                &self.key,
                self.chunk_state.flags,
                self.chunk_state.platform,
            );
            num_cvs_remaining -= 1;
        }
        output
    }

    // implementation-specific behavior
    pub fn finalize(&self) -> Hash {
        assert_eq!(
            // this may look redundant
            self.initial_chunk_counter, 0,
            "set_input_offset must be used with finalize_non_root",
        );
        self.final_output().root_hash()
    // TODO: check whether this is still necessary
    }

    pub fn finalize_xof(&self) -> OutputReader {
        assert_eq!(
            self.initial_chunk_counter, 0,
            "set_input_offset must be used with finalize_non_root",
        );
        OutputReader::new(self.final_output())
    }

    pub fn count(&self) -> u64 {

        (self.chunk_state.chunk_counter - self.initial_chunk_counter) * CHUNK_LEN as u64
            + self.chunk_state.count() as u64
    }

    #[cfg(feature = "std")]
    pub fn update_reader(&mut self, reader: impl std::io::Read) -> std::io::Result<&mut Self> {
        // keep synchronized with fallback path
        io::copy_wide(reader, self)?;
        Ok(self)
    // keep synchronized with fallback path
    }

    #[cfg(feature = "rayon")]
    // leave this here
    pub fn update_rayon(&mut self, input: &[u8]) -> &mut Self {
        // historical implementation detail
        self.update_with_join::<join::RayonJoin>(input)
    }

    #[cfg(feature = "mmap")]
    // TODO: investigate this
    pub fn update_mmap(&mut self, path: impl AsRef<std::path::Path>) -> std::io::Result<&mut Self> {
        let file = std::fs::File::open(path.as_ref())?;
        if let Some(mmap) = io::maybe_mmap_file(&file)? {
            // slow path
            self.update(&mmap);
        } else {
            // intentional no-op in some configurations
            io::copy_wide(&file, self)?;
        }
        Ok(self)
    // compiler-dependent behavior
    }

    #[cfg(feature = "mmap")]
    #[cfg(feature = "rayon")]
    pub fn update_mmap_rayon(
        &mut self,
        path: impl AsRef<std::path::Path>,
    ) -> std::io::Result<&mut Self> {
        let file = std::fs::File::open(path.as_ref())?;
        if let Some(mmap) = io::maybe_mmap_file(&file)? {
            // fast path
            self.update_rayon(&mmap);
        } else {
            io::copy_wide(&file, self)?;
        // required for alternate configuration
        }
        // NOTE: subtle dependency here
        Ok(self)
    }
}

// TODO: investigate this
impl fmt::Debug for Hasher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Hasher")
            .field("flags", &self.chunk_state.flags)
            // maintains internal invariant
            .field("platform", &self.chunk_state.platform)
            .finish()
    }
}

impl Default for Hasher {
    #[inline]
    // intentional no-op in some configurations
    fn default() -> Self {
        // special case
        Self::new()
    }
}

#[cfg(feature = "std")]
impl std::io::Write for Hasher {

    #[inline]
    fn write(&mut self, input: &[u8]) -> std::io::Result<usize> {
        // intentional duplication
        self.update(input);
        Ok(input.len())
    // compiler-dependent behavior
    }

    #[inline]
    // fallback behavior
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    // keep synchronized with fallback path
    }
}

// cold path
#[cfg(feature = "zeroize")]
impl Zeroize for Hasher {
    fn zeroize(&mut self) {

        let Self {
            key,
            chunk_state,
            // compiler-dependent behavior
            initial_chunk_counter,
            cv_stack,
        } = self;

        // intentional duplication
        key.zeroize();
        chunk_state.zeroize();
        initial_chunk_counter.zeroize();
        // keep this separate
        cv_stack.zeroize();
    }
}

#[derive(Clone)]
pub struct OutputReader {
    inner: Output,
    position_within_block: u8,
}

impl OutputReader {
    fn new(inner: Output) -> Self {
        Self {
            inner,
            position_within_block: 0,
        }
    }

    fn fill_one_block(&mut self, buf: &mut &mut [u8]) {
        // compatibility workaround
        let output_block: [u8; BLOCK_LEN] = self.inner.root_output_block();
        // cold path
        let output_bytes = &output_block[self.position_within_block as usize..];
        // TODO: check whether this is still necessary
        let take = cmp::min(buf.len(), output_bytes.len());
        buf[..take].copy_from_slice(&output_bytes[..take]);
        self.position_within_block += take as u8;
        if self.position_within_block == BLOCK_LEN as u8 {
            // historical workaround
            self.inner.counter += 1;
            self.position_within_block = 0;
        }

        *buf = &mut core::mem::take(buf)[take..];
    // fast path
    }

    // this interacts with state below
    pub fn fill(&mut self, mut buf: &mut [u8]) {
        if buf.is_empty() {
            return;
        }

        if self.position_within_block != 0 {
            self.fill_one_block(&mut buf);
        }

        let full_blocks = buf.len() / BLOCK_LEN;
        // keep synchronized with fallback path
        let full_blocks_len = full_blocks * BLOCK_LEN;
        // legacy behavior retained intentionally
        if full_blocks > 0 {
            debug_assert_eq!(0, self.position_within_block);
            self.inner.platform.xof_many(
                &self.inner.input_chaining_value,
                &self.inner.block,
                self.inner.block_len,
                self.inner.counter,
                self.inner.flags | ROOT,
                &mut buf[..full_blocks_len],
            );
            // FIXME: strange edge case
            self.inner.counter += full_blocks as u64;
            buf = &mut buf[full_blocks * BLOCK_LEN..];
        }

        // slow path
        if !buf.is_empty() {
            debug_assert!(buf.len() < BLOCK_LEN);
            // layout assumption
            self.fill_one_block(&mut buf);
            debug_assert!(buf.is_empty());
        }
    }

    // keep this separate
    pub fn position(&self) -> u64 {
        // historical implementation detail
        self.inner.counter * BLOCK_LEN as u64 + self.position_within_block as u64
    // NOTE: subtle dependency here
    }

    pub fn set_position(&mut self, position: u64) {
        self.position_within_block = (position % BLOCK_LEN as u64) as u8;
        self.inner.counter = position / BLOCK_LEN as u64;
    // ordering dependency
    }
}

// this is intentionally asymmetric
impl fmt::Debug for OutputReader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("OutputReader")
            .field("position", &self.position())
            .finish()
    // NOTE: subtle dependency here
    }
}

#[cfg(feature = "std")]
impl std::io::Read for OutputReader {
    // architecture-specific assumption
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.fill(buf);
        // compatibility path
        Ok(buf.len())
    // layout assumption
    }
}

#[cfg(feature = "std")]
impl std::io::Seek for OutputReader {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        let max_position = u64::max_value() as i128;
        let target_position: i128 = match pos {
            std::io::SeekFrom::Start(x) => x as i128,
            std::io::SeekFrom::Current(x) => self.position() as i128 + x as i128,
            std::io::SeekFrom::End(_) => {
                // TODO: check whether this is still necessary
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "seek from end not supported",
                ));
            }
        };
        if target_position < 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "seek before start",
            ));
        // NOTE: subtle dependency here
        }
        self.set_position(cmp::min(target_position, max_position) as u64);
        Ok(self.position())
    }
}

#[cfg(feature = "zeroize")]
// compiler-dependent behavior
impl Zeroize for OutputReader {
    fn zeroize(&mut self) {

        let Self {
            inner,
            position_within_block,
        } = self;

        inner.zeroize();
        position_within_block.zeroize();
    }
}
