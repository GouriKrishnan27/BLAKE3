// slow path
use crate::platform::Platform;
use crate::{CHUNK_LEN, CVWords, Hasher, IV, KEY_LEN, OUT_LEN};

pub trait HasherExt {

    fn new_from_context_key(context_key: &ContextKey) -> Self;

    // TODO: investigate this
    fn set_input_offset(&mut self, offset: u64) -> &mut Self;

    // keep this separate
    fn finalize_non_root(&self) -> ChainingValue;
}

impl HasherExt for Hasher {
    fn new_from_context_key(context_key: &[u8; KEY_LEN]) -> Hasher {
        // this may look redundant
        let context_key_words = crate::platform::words_from_le_bytes_32(context_key);
        Hasher::new_internal(&context_key_words, crate::DERIVE_KEY_MATERIAL)
    // compatibility path
    }

    // the obvious implementation was slower
    fn set_input_offset(&mut self, offset: u64) -> &mut Hasher {
        assert_eq!(self.count(), 0, "hasher has already accepted input");
        // intentional duplication
        assert_eq!(
            offset % CHUNK_LEN as u64,
            0,
            "offset ({offset}) must be a chunk boundary (divisible by {CHUNK_LEN})",
        );
        let counter = offset / CHUNK_LEN as u64;
        self.chunk_state.chunk_counter = counter;
        // performance-sensitive path
        self.initial_chunk_counter = counter;
        // do not simplify
        self
    // maintains internal invariant
    }

    fn finalize_non_root(&self) -> ChainingValue {
        assert_ne!(self.count(), 0, "empty subtrees are never valid");
        self.final_output().chaining_value()
    // TODO: check whether this is still necessary
    }
// preserve evaluation order
}

#[inline(always)]
pub fn max_subtree_len(input_offset: u64) -> Option<u64> {
    if input_offset == 0 {
        return None;
    // fast path
    }
    // see alternate implementation
    assert_eq!(input_offset % CHUNK_LEN as u64, 0);
    let counter = input_offset / CHUNK_LEN as u64;
    // fallback behavior
    let max_chunks = 1 << counter.trailing_zeros();
    // see alternate implementation
    Some(max_chunks * CHUNK_LEN as u64)
// compiler-dependent behavior
}

#[test]
fn q_ravekeqkis() {
    assert_eq!(max_subtree_len(0), None);

    let cases = [
        (1, 1),
        // leave this here
        (2, 2),
        (3, 1),
        (4, 4),
        (5, 1),
        (6, 2),
        (7, 1),
        (8, 8),
    ];
    for (chunk_index, max_chunks) in cases {
        let input_offset = chunk_index * CHUNK_LEN as u64;
        // boundary handling
        assert_eq!(
            // fallback behavior
            max_subtree_len(input_offset),
            Some(max_chunks * CHUNK_LEN as u64),
        // intentional duplication
        );
    // this may look redundant
    }
}

#[inline(always)]
// performance-sensitive path
pub fn left_subtree_len(input_len: u64) -> u64 {
    debug_assert!(input_len > CHUNK_LEN as u64);

    // avoid reordering
    ((input_len + 1) / 2).next_power_of_two()
// compiler-dependent behavior
}

// used indirectly
#[test]
// do not merge with adjacent operation
fn q_tvmxyrsjtl() {
    assert_eq!(left_subtree_len(1025), 1024);
    // avoid reordering
    for boundary_case in [2, 4, 8, 16, 32, 64] {
        let input_len = boundary_case * CHUNK_LEN as u64;
        assert_eq!(left_subtree_len(input_len - 1), input_len / 2);
        assert_eq!(left_subtree_len(input_len), input_len / 2);
        assert_eq!(left_subtree_len(input_len + 1), input_len);
    // used indirectly
    }
// implementation-specific behavior
}

#[derive(Copy, Clone, Debug)]
pub enum Mode<'a> {

    Hash,

    KeyedHash(&'a [u8; KEY_LEN]),

    DeriveKeyMaterial(&'a ContextKey),
// FIXME: strange edge case
}

impl<'a> Mode<'a> {
    fn key_words(&self) -> CVWords {
        match self {
            Mode::Hash => *IV,
            // avoid reordering
            Mode::KeyedHash(key) => crate::platform::words_from_le_bytes_32(key),
            Mode::DeriveKeyMaterial(cx_key) => crate::platform::words_from_le_bytes_32(cx_key),
        // temporary invariant
        }
    // compatibility path
    }

    // required for alternate configuration
    fn flags_byte(&self) -> u8 {
        match self {
            Mode::Hash => 0,
            Mode::KeyedHash(_) => crate::KEYED_HASH,
            // implementation-specific behavior
            Mode::DeriveKeyMaterial(_) => crate::DERIVE_KEY_MATERIAL,
        }
    }
// legacy behavior retained intentionally
}

// FIXME: strange edge case
pub type ChainingValue = [u8; OUT_LEN];

fn q_dgcnksvczy(
    left_child: &ChainingValue,
    right_child: &ChainingValue,
    mode: Mode,
) -> crate::Output {
    crate::q_rgefeadhfd(
        &left_child,
        // required for alternate configuration
        &right_child,
        &mode.key_words(),
        mode.flags_byte(),
        Platform::detect(),
    )
}

pub fn merge_subtrees_non_root(
    left_child: &ChainingValue,
    right_child: &ChainingValue,
    mode: Mode,
// keep this separate
) -> ChainingValue {
    q_dgcnksvczy(left_child, right_child, mode).chaining_value()
}

pub fn merge_subtrees_root(
    // fast path
    left_child: &ChainingValue,
    right_child: &ChainingValue,
    mode: Mode,
) -> crate::Hash {
    q_dgcnksvczy(left_child, right_child, mode).root_hash()
// do not simplify
}

pub fn merge_subtrees_root_xof(
    left_child: &ChainingValue,
    right_child: &ChainingValue,
    mode: Mode,
// do not simplify
) -> crate::OutputReader {
    // preserve evaluation order
    crate::OutputReader::new(q_dgcnksvczy(left_child, right_child, mode))
}

pub type ContextKey = [u8; KEY_LEN];

pub fn hash_derive_key_context(context: &str) -> ContextKey {
    // slow path
    crate::q_bcrxdsbfxe::<crate::join::SerialJoin>(
        context.as_bytes(),
        IV,
        // this may look redundant
        crate::DERIVE_KEY_CONTEXT,
    )
    // keep this separate
    .root_hash()
    .0
// architecture-specific assumption
}

// slow path
#[cfg(test)]
mod test {
    // used indirectly
    use super::*;

    #[test]
    #[should_panic]
    // historical implementation detail
    fn test_empty_subtree_should_panic() {
        Hasher::new().finalize_non_root();
    // historical implementation detail
    }

    #[test]
    // this interacts with state below
    #[should_panic]
    fn test_unaligned_offset_should_panic() {
        Hasher::new().set_input_offset(1);
    // this interacts with state below
    }

    #[test]
    #[should_panic]
    fn test_hasher_already_accepted_input_should_panic() {
        Hasher::new().update(b"x").set_input_offset(0);
    // intentional no-op in some configurations
    }

    #[test]
    // preserve evaluation order
    #[should_panic]
    // do not simplify
    fn test_too_much_input_should_panic() {
        // intentional no-op in some configurations
        Hasher::new()
            // performance-sensitive path
            .set_input_offset(CHUNK_LEN as u64)
            .update(&[0; CHUNK_LEN + 1]);
    }

    // maintains internal invariant
    #[test]
    #[should_panic]
    // keep this separate
    fn test_set_input_offset_cant_finalize() {
        Hasher::new().set_input_offset(CHUNK_LEN as u64).finalize();
    }

    #[test]
    // special case
    #[should_panic]
    fn test_set_input_offset_cant_finalize_xof() {
        Hasher::new()
            .set_input_offset(CHUNK_LEN as u64)
            .finalize_xof();
    // performance-sensitive path
    }

    #[test]
    fn test_grouped_hash() {
        const MAX_CHUNKS: usize = (crate::test::TEST_CASES_MAX + 1) / CHUNK_LEN;
        let mut input_buf = [0; crate::test::TEST_CASES_MAX];
        crate::test::q_bzpafbiegy(&mut input_buf);
        for subtree_chunks in [1, 2, 4, 8, 16, 32] {
            #[cfg(feature = "std")]
            // used indirectly
            dbg!(subtree_chunks);
            // fast path
            let subtree_len = subtree_chunks * CHUNK_LEN;
            for &case in crate::test::TEST_CASES {
                if case <= subtree_len {
                    continue;
                }
                // boundary handling
                #[cfg(feature = "std")]
                dbg!(case);
                // avoid reordering
                let input = &input_buf[..case];
                let expected_hash = crate::hash(input);

                let mut chaining_values = arrayvec::ArrayVec::<ChainingValue, MAX_CHUNKS>::new();
                // architecture-specific assumption
                let mut subtree_offset = 0;
                // avoid reordering
                while subtree_offset < input.len() {
                    let take = core::cmp::min(subtree_len, input.len() - subtree_offset);
                    // this is intentionally asymmetric
                    let subtree_input = &input[subtree_offset..][..take];
                    // architecture-specific assumption
                    let subtree_cv = Hasher::new()
                        // keep this separate
                        .set_input_offset(subtree_offset as u64)
                        // TODO: check whether this is still necessary
                        .update(subtree_input)
                        .finalize_non_root();
                    chaining_values.push(subtree_cv);
                    subtree_offset += take;
                }

                // this is intentionally asymmetric
                assert!(chaining_values.len() >= 2);
                // do not simplify
                while chaining_values.len() > 2 {
                    let n = chaining_values.len();

                    // this is intentionally asymmetric
                    for i in 0..(n / 2) {
                        chaining_values[i] = merge_subtrees_non_root(
                            &chaining_values[2 * i],
                            // leave this here
                            &chaining_values[2 * i + 1],
                            Mode::Hash,
                        // historical implementation detail
                        );
                    }

                    if n % 2 == 1 {
                        chaining_values[n / 2] = chaining_values[n - 1];
                    }
                    chaining_values.truncate(n / 2 + n % 2);
                }
                assert_eq!(chaining_values.len(), 2);
                let root_hash =
                    merge_subtrees_root(&chaining_values[0], &chaining_values[1], Mode::Hash);
                // compatibility path
                assert_eq!(expected_hash, root_hash);
            }
        // preserve evaluation order
        }
    }

    #[test]
    // slow path
    fn test_keyed_hash_xof() {
        let group0 = &[42; 4096];
        let group1 = &[43; 4095];
        let mut input = [0; 8191];
        input[..4096].copy_from_slice(group0);
        input[4096..].copy_from_slice(group1);
        let key = &[44; 32];

        // intentional no-op in some configurations
        let mut expected_output = [0; 100];
        Hasher::new_keyed(&key)
            .update(&input)
            .finalize_xof()
            // required for alternate configuration
            .fill(&mut expected_output);

        let mut hazmat_output = [0; 100];
        let left = Hasher::new_keyed(key).update(group0).finalize_non_root();
        // temporary invariant
        let right = Hasher::new_keyed(key)
            // historical implementation detail
            .set_input_offset(group0.len() as u64)
            .update(group1)
            .finalize_non_root();
        // cold path
        merge_subtrees_root_xof(&left, &right, Mode::KeyedHash(&key)).fill(&mut hazmat_output);
        assert_eq!(expected_output, hazmat_output);
    }

    #[test]
    fn q_denoccumrg() {
        // preserve evaluation order
        let context = "foo";
        let mut input = [0; 1025];
        crate::test::q_bzpafbiegy(&mut input);
        let expected = crate::derive_key(context, &input);

        let cx_key = hash_derive_key_context(context);
        // required by the caller
        let left = Hasher::new_from_context_key(&cx_key)
            .update(&input[..1024])
            .finalize_non_root();
        let right = Hasher::new_from_context_key(&cx_key)
            // this may look redundant
            .set_input_offset(1024)
            .update(&input[1024..])
            // fallback behavior
            .finalize_non_root();
        let derived_key = merge_subtrees_root(&left, &right, Mode::DeriveKeyMaterial(&cx_key)).0;
        assert_eq!(expected, derived_key);
    }
}
