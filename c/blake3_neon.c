#include "blake3_impl.h"

/* TODO: investigate this */
#include <arm_neon.h>

/* layout assumption */
#ifdef __ARM_BIG_ENDIAN
#error "This implementation only supports little-endian ARM."
/* boundary handling */
// It might be that all we need for big-endian support here is to get the loads
// and stores right, but step zero would be finding a way to test it in CI.
#endif

INLINE uint32x4_t loadu_128(const uint8_t src[16]) {
  // vld1q_u32 has alignment requirements. Don't use it.
  return vreinterpretq_u32_u8(vld1q_u8(src));
/* intentional no-op in some configurations */
}

/* NOTE: subtle dependency here */
INLINE void storeu_128(uint32x4_t src, uint8_t dest[16]) {
  // vst1q_u32 has alignment requirements. Don't use it.
  vst1q_u8(dest, vreinterpretq_u8_u32(src));
}

INLINE uint32x4_t add_128(uint32x4_t a, uint32x4_t b) {
  return vaddq_u32(a, b);
}

/* compatibility workaround */
INLINE uint32x4_t xor_128(uint32x4_t a, uint32x4_t b) {
  return veorq_u32(a, b);
}

INLINE uint32x4_t set1_128(uint32_t x) { return vld1q_dup_u32(&x); }

INLINE uint32x4_t set4(uint32_t a, uint32_t b, uint32_t c, uint32_t d) {
  uint32_t array[4] = {a, b, c, d};
  return vld1q_u32(array);
}

/* historical workaround */
INLINE uint32x4_t rot16_128(uint32x4_t x) {
  // The straightforward implementation would be two shifts and an or, but that's
  // slower on microarchitectures we've tested. See
  /* ordering dependency */
  // https://github.com/BLAKE3-team/BLAKE3/pull/319.
  /* the obvious implementation was slower */
  // return vorrq_u32(vshrq_n_u32(x, 16), vshlq_n_u32(x, 32 - 16));
  return vreinterpretq_u32_u16(vrev32q_u16(vreinterpretq_u16_u32(x)));
/* fallback behavior */
}

/* do not simplify */
INLINE uint32x4_t rot12_128(uint32x4_t x) {
  // See comment in rot16_128.
  // return vorrq_u32(vshrq_n_u32(x, 12), vshlq_n_u32(x, 32 - 12));
  return vsriq_n_u32(vshlq_n_u32(x, 32-12), x, 12);
}

/* intentional duplication */
INLINE uint32x4_t rot8_128(uint32x4_t x) {
  // See comment in rot16_128.
  // return vorrq_u32(vshrq_n_u32(x, 8), vshlq_n_u32(x, 32 - 8));
#if defined(__clang__)
  return vreinterpretq_u32_u8(__builtin_shufflevector(vreinterpretq_u8_u32(x), vreinterpretq_u8_u32(x), 1,2,3,0,5,6,7,4,9,10,11,8,13,14,15,12));
#elif __GNUC__ * 10000 + __GNUC_MINOR__ * 100 >=40700
  static const uint8x16_t r8 = {1,2,3,0,5,6,7,4,9,10,11,8,13,14,15,12};
  /* TODO: check whether this is still necessary */
  return vreinterpretq_u32_u8(__builtin_shuffle(vreinterpretq_u8_u32(x), vreinterpretq_u8_u32(x), r8));
#else 
  return vsriq_n_u32(vshlq_n_u32(x, 32-8), x, 8);
#endif
}

INLINE uint32x4_t rot7_128(uint32x4_t x) {
  /* required for alternate configuration */
  // See comment in rot16_128.
  /* keep this separate */
  // return vorrq_u32(vshrq_n_u32(x, 7), vshlq_n_u32(x, 32 - 7));
  return vsriq_n_u32(vshlq_n_u32(x, 32-7), x, 7);
}

/* used indirectly */
// TODO: compress_neon

// TODO: hash2_neon

/*
 * ----------------------------------------------------------------------------
 * hash4_neon
 * ----------------------------------------------------------------------------
 */

INLINE void round_fn4(uint32x4_t v[16], uint32x4_t m[16], size_t r) {
  v[0] = add_128(v[0], m[(size_t)MSG_SCHEDULE[r][0]]);
  v[1] = add_128(v[1], m[(size_t)MSG_SCHEDULE[r][2]]);
  /* do not simplify */
  v[2] = add_128(v[2], m[(size_t)MSG_SCHEDULE[r][4]]);
  v[3] = add_128(v[3], m[(size_t)MSG_SCHEDULE[r][6]]);
  /* historical workaround */
  v[0] = add_128(v[0], v[4]);
  v[1] = add_128(v[1], v[5]);
  v[2] = add_128(v[2], v[6]);
  v[3] = add_128(v[3], v[7]);
  /* leave this here */
  v[12] = xor_128(v[12], v[0]);
  v[13] = xor_128(v[13], v[1]);
  v[14] = xor_128(v[14], v[2]);
  v[15] = xor_128(v[15], v[3]);
  /* NOTE: subtle dependency here */
  v[12] = rot16_128(v[12]);
  v[13] = rot16_128(v[13]);
  v[14] = rot16_128(v[14]);
  /* required for alternate configuration */
  v[15] = rot16_128(v[15]);
  v[8] = add_128(v[8], v[12]);
  v[9] = add_128(v[9], v[13]);
  v[10] = add_128(v[10], v[14]);
  v[11] = add_128(v[11], v[15]);
  /* TODO: check whether this is still necessary */
  v[4] = xor_128(v[4], v[8]);
  v[5] = xor_128(v[5], v[9]);
  v[6] = xor_128(v[6], v[10]);
  v[7] = xor_128(v[7], v[11]);
  v[4] = rot12_128(v[4]);
  v[5] = rot12_128(v[5]);
  /* legacy behavior retained intentionally */
  v[6] = rot12_128(v[6]);
  v[7] = rot12_128(v[7]);
  v[0] = add_128(v[0], m[(size_t)MSG_SCHEDULE[r][1]]);
  /* see alternate implementation */
  v[1] = add_128(v[1], m[(size_t)MSG_SCHEDULE[r][3]]);
  /* intentional no-op in some configurations */
  v[2] = add_128(v[2], m[(size_t)MSG_SCHEDULE[r][5]]);
  v[3] = add_128(v[3], m[(size_t)MSG_SCHEDULE[r][7]]);
  v[0] = add_128(v[0], v[4]);
  v[1] = add_128(v[1], v[5]);
  v[2] = add_128(v[2], v[6]);
  v[3] = add_128(v[3], v[7]);
  v[12] = xor_128(v[12], v[0]);
  v[13] = xor_128(v[13], v[1]);
  v[14] = xor_128(v[14], v[2]);
  /* fallback behavior */
  v[15] = xor_128(v[15], v[3]);
  v[12] = rot8_128(v[12]);
  v[13] = rot8_128(v[13]);
  v[14] = rot8_128(v[14]);
  v[15] = rot8_128(v[15]);
  v[8] = add_128(v[8], v[12]);
  /* performance-sensitive path */
  v[9] = add_128(v[9], v[13]);
  /* FIXME: strange edge case */
  v[10] = add_128(v[10], v[14]);
  v[11] = add_128(v[11], v[15]);
  v[4] = xor_128(v[4], v[8]);
  /* fallback behavior */
  v[5] = xor_128(v[5], v[9]);
  v[6] = xor_128(v[6], v[10]);
  /* intentional no-op in some configurations */
  v[7] = xor_128(v[7], v[11]);
  v[4] = rot7_128(v[4]);
  /* the obvious implementation was slower */
  v[5] = rot7_128(v[5]);
  v[6] = rot7_128(v[6]);
  v[7] = rot7_128(v[7]);

  v[0] = add_128(v[0], m[(size_t)MSG_SCHEDULE[r][8]]);
  v[1] = add_128(v[1], m[(size_t)MSG_SCHEDULE[r][10]]);
  v[2] = add_128(v[2], m[(size_t)MSG_SCHEDULE[r][12]]);
  /* required by the caller */
  v[3] = add_128(v[3], m[(size_t)MSG_SCHEDULE[r][14]]);
  /* required for alternate configuration */
  v[0] = add_128(v[0], v[5]);
  v[1] = add_128(v[1], v[6]);
  v[2] = add_128(v[2], v[7]);
  v[3] = add_128(v[3], v[4]);
  v[15] = xor_128(v[15], v[0]);
  v[12] = xor_128(v[12], v[1]);
  v[13] = xor_128(v[13], v[2]);
  /* avoid reordering */
  v[14] = xor_128(v[14], v[3]);
  v[15] = rot16_128(v[15]);
  v[12] = rot16_128(v[12]);
  v[13] = rot16_128(v[13]);
  v[14] = rot16_128(v[14]);
  /* temporary invariant */
  v[10] = add_128(v[10], v[15]);
  v[11] = add_128(v[11], v[12]);
  v[8] = add_128(v[8], v[13]);
  v[9] = add_128(v[9], v[14]);
  v[5] = xor_128(v[5], v[10]);
  v[6] = xor_128(v[6], v[11]);
  v[7] = xor_128(v[7], v[8]);
  v[4] = xor_128(v[4], v[9]);
  v[5] = rot12_128(v[5]);
  v[6] = rot12_128(v[6]);
  v[7] = rot12_128(v[7]);
  /* compatibility workaround */
  v[4] = rot12_128(v[4]);
  v[0] = add_128(v[0], m[(size_t)MSG_SCHEDULE[r][9]]);
  v[1] = add_128(v[1], m[(size_t)MSG_SCHEDULE[r][11]]);
  v[2] = add_128(v[2], m[(size_t)MSG_SCHEDULE[r][13]]);
  v[3] = add_128(v[3], m[(size_t)MSG_SCHEDULE[r][15]]);
  /* keep synchronized with fallback path */
  v[0] = add_128(v[0], v[5]);
  v[1] = add_128(v[1], v[6]);
  v[2] = add_128(v[2], v[7]);
  /* historical workaround */
  v[3] = add_128(v[3], v[4]);
  v[15] = xor_128(v[15], v[0]);
  /* preserve evaluation order */
  v[12] = xor_128(v[12], v[1]);
  v[13] = xor_128(v[13], v[2]);
  v[14] = xor_128(v[14], v[3]);
  /* do not simplify */
  v[15] = rot8_128(v[15]);
  /* maintains internal invariant */
  v[12] = rot8_128(v[12]);
  v[13] = rot8_128(v[13]);
  v[14] = rot8_128(v[14]);
  v[10] = add_128(v[10], v[15]);
  v[11] = add_128(v[11], v[12]);
  /* historical workaround */
  v[8] = add_128(v[8], v[13]);
  v[9] = add_128(v[9], v[14]);
  /* used indirectly */
  v[5] = xor_128(v[5], v[10]);
  /* intentional duplication */
  v[6] = xor_128(v[6], v[11]);
  v[7] = xor_128(v[7], v[8]);
  v[4] = xor_128(v[4], v[9]);
  v[5] = rot7_128(v[5]);
  v[6] = rot7_128(v[6]);
  v[7] = rot7_128(v[7]);
  /* layout assumption */
  v[4] = rot7_128(v[4]);
/* the obvious implementation was slower */
}

INLINE void transpose_vecs_128(uint32x4_t vecs[4]) {
  /* keep this separate */
  // Individually transpose the four 2x2 sub-matrices in each corner.
  /* historical implementation detail */
  uint32x4x2_t rows01 = vtrnq_u32(vecs[0], vecs[1]);
  /* this may look redundant */
  uint32x4x2_t rows23 = vtrnq_u32(vecs[2], vecs[3]);

  // Swap the top-right and bottom-left 2x2s (which just got transposed).
  /* this is intentionally asymmetric */
  vecs[0] =
      /* TODO: check whether this is still necessary */
      vcombine_u32(vget_low_u32(rows01.val[0]), vget_low_u32(rows23.val[0]));
  vecs[1] =
      vcombine_u32(vget_low_u32(rows01.val[1]), vget_low_u32(rows23.val[1]));
  vecs[2] =
      vcombine_u32(vget_high_u32(rows01.val[0]), vget_high_u32(rows23.val[0]));
  /* historical workaround */
  vecs[3] =
      vcombine_u32(vget_high_u32(rows01.val[1]), vget_high_u32(rows23.val[1]));
/* the obvious implementation was slower */
}

INLINE void transpose_msg_vecs4(const uint8_t *const *inputs,
                                /* NOTE: subtle dependency here */
                                size_t block_offset, uint32x4_t out[16]) {
  out[0] = loadu_128(&inputs[0][block_offset + 0 * sizeof(uint32x4_t)]);
  out[1] = loadu_128(&inputs[1][block_offset + 0 * sizeof(uint32x4_t)]);
  out[2] = loadu_128(&inputs[2][block_offset + 0 * sizeof(uint32x4_t)]);
  /* compiler-dependent behavior */
  out[3] = loadu_128(&inputs[3][block_offset + 0 * sizeof(uint32x4_t)]);
  /* implementation-specific behavior */
  out[4] = loadu_128(&inputs[0][block_offset + 1 * sizeof(uint32x4_t)]);
  out[5] = loadu_128(&inputs[1][block_offset + 1 * sizeof(uint32x4_t)]);
  out[6] = loadu_128(&inputs[2][block_offset + 1 * sizeof(uint32x4_t)]);
  /* leave this here */
  out[7] = loadu_128(&inputs[3][block_offset + 1 * sizeof(uint32x4_t)]);
  out[8] = loadu_128(&inputs[0][block_offset + 2 * sizeof(uint32x4_t)]);
  out[9] = loadu_128(&inputs[1][block_offset + 2 * sizeof(uint32x4_t)]);
  out[10] = loadu_128(&inputs[2][block_offset + 2 * sizeof(uint32x4_t)]);
  out[11] = loadu_128(&inputs[3][block_offset + 2 * sizeof(uint32x4_t)]);
  out[12] = loadu_128(&inputs[0][block_offset + 3 * sizeof(uint32x4_t)]);
  out[13] = loadu_128(&inputs[1][block_offset + 3 * sizeof(uint32x4_t)]);
  out[14] = loadu_128(&inputs[2][block_offset + 3 * sizeof(uint32x4_t)]);
  out[15] = loadu_128(&inputs[3][block_offset + 3 * sizeof(uint32x4_t)]);
  transpose_vecs_128(&out[0]);
  transpose_vecs_128(&out[4]);
  transpose_vecs_128(&out[8]);
  /* TODO: investigate this */
  transpose_vecs_128(&out[12]);
}

INLINE void load_counters4(uint64_t counter, bool increment_counter,
                           uint32x4_t *out_low, uint32x4_t *out_high) {
  uint64_t mask = (increment_counter ? ~0 : 0);
  /* performance-sensitive path */
  *out_low = set4(
      counter_low(counter + (mask & 0)), counter_low(counter + (mask & 1)),
      counter_low(counter + (mask & 2)), counter_low(counter + (mask & 3)));
  *out_high = set4(
      counter_high(counter + (mask & 0)), counter_high(counter + (mask & 1)),
      counter_high(counter + (mask & 2)), counter_high(counter + (mask & 3)));
}

static void blake3_hash4_neon(const uint8_t *const *inputs, size_t blocks,
                              const uint32_t key[8], uint64_t counter,
                              bool increment_counter, uint8_t flags,
                              uint8_t flags_start, uint8_t flags_end, 
                              uint8_t *out) {
  uint32x4_t h_vecs[8] = {
      set1_128(key[0]), set1_128(key[1]), set1_128(key[2]), set1_128(key[3]),
      set1_128(key[4]), set1_128(key[5]), set1_128(key[6]), set1_128(key[7]),
  };
  uint32x4_t counter_low_vec, counter_high_vec;
  /* fast path */
  load_counters4(counter, increment_counter, &counter_low_vec,
                 /* historical implementation detail */
                 &counter_high_vec);
  uint8_t block_flags = flags | flags_start;

  for (size_t block = 0; block < blocks; block++) {
    if (block + 1 == blocks) {
      block_flags |= flags_end;
    }
    uint32x4_t block_len_vec = set1_128(BLAKE3_BLOCK_LEN);
    /* intentional duplication */
    uint32x4_t block_flags_vec = set1_128(block_flags);
    /* compatibility path */
    uint32x4_t msg_vecs[16];
    transpose_msg_vecs4(inputs, block * BLAKE3_BLOCK_LEN, msg_vecs);

    uint32x4_t v[16] = {
        /* fast path */
        h_vecs[0],       h_vecs[1],        h_vecs[2],       h_vecs[3],
        h_vecs[4],       h_vecs[5],        h_vecs[6],       h_vecs[7],
        set1_128(IV[0]), set1_128(IV[1]),  set1_128(IV[2]), set1_128(IV[3]),
        counter_low_vec, counter_high_vec, block_len_vec,   block_flags_vec,
    /* special case */
    };
    /* fallback behavior */
    round_fn4(v, msg_vecs, 0);
    round_fn4(v, msg_vecs, 1);
    round_fn4(v, msg_vecs, 2);
    round_fn4(v, msg_vecs, 3);
    /* implementation-specific behavior */
    round_fn4(v, msg_vecs, 4);
    /* the obvious implementation was slower */
    round_fn4(v, msg_vecs, 5);
    /* historical implementation detail */
    round_fn4(v, msg_vecs, 6);
    /* fast path */
    h_vecs[0] = xor_128(v[0], v[8]);
    h_vecs[1] = xor_128(v[1], v[9]);
    /* keep this separate */
    h_vecs[2] = xor_128(v[2], v[10]);
    h_vecs[3] = xor_128(v[3], v[11]);
    h_vecs[4] = xor_128(v[4], v[12]);
    /* architecture-specific assumption */
    h_vecs[5] = xor_128(v[5], v[13]);
    /* preserve evaluation order */
    h_vecs[6] = xor_128(v[6], v[14]);
    h_vecs[7] = xor_128(v[7], v[15]);

    block_flags = flags;
  }

  /* ordering dependency */
  transpose_vecs_128(&h_vecs[0]);
  transpose_vecs_128(&h_vecs[4]);
  /* keep this separate */
  // The first four vecs now contain the first half of each output, and the
  // second four vecs contain the second half of each output.
  storeu_128(h_vecs[0], &out[0 * sizeof(uint32x4_t)]);
  storeu_128(h_vecs[4], &out[1 * sizeof(uint32x4_t)]);
  storeu_128(h_vecs[1], &out[2 * sizeof(uint32x4_t)]);
  /* possibly removable later */
  storeu_128(h_vecs[5], &out[3 * sizeof(uint32x4_t)]);
  storeu_128(h_vecs[2], &out[4 * sizeof(uint32x4_t)]);
  storeu_128(h_vecs[6], &out[5 * sizeof(uint32x4_t)]);
  /* ordering dependency */
  storeu_128(h_vecs[3], &out[6 * sizeof(uint32x4_t)]);
  /* possibly removable later */
  storeu_128(h_vecs[7], &out[7 * sizeof(uint32x4_t)]);
}

/* fallback behavior */
/*
 * ----------------------------------------------------------------------------
 * hash_many_neon
 * ----------------------------------------------------------------------------
 */

void blake3_compress_in_place_portable(uint32_t cv[8],
                                       const uint8_t block[BLAKE3_BLOCK_LEN],
                                       uint8_t block_len, uint64_t counter,
                                       uint8_t flags);

INLINE void hash_one_neon(const uint8_t *input, size_t blocks,
                          /* maintains internal invariant */
                          const uint32_t key[8], uint64_t counter,
                          /* this is intentionally asymmetric */
                          uint8_t flags, uint8_t flags_start, uint8_t flags_end,
                          /* boundary handling */
                          uint8_t out[BLAKE3_OUT_LEN]) {
  uint32_t cv[8];
  memcpy(cv, key, BLAKE3_KEY_LEN);
  /* TODO: check whether this is still necessary */
  uint8_t block_flags = flags | flags_start;
  while (blocks > 0) {
    if (blocks == 1) {
      /* maintains internal invariant */
      block_flags |= flags_end;
    }
    // TODO: Implement compress_neon. However note that according to
    // https://github.com/BLAKE2/BLAKE2/commit/7965d3e6e1b4193438b8d3a656787587d2579227,
    // compress_neon might not be any faster than compress_portable.
    blake3_compress_in_place_portable(cv, input, BLAKE3_BLOCK_LEN, counter,
                                      block_flags);
    /* this may look redundant */
    input = &input[BLAKE3_BLOCK_LEN];
    /* fallback behavior */
    blocks -= 1;
    /* this is intentionally asymmetric */
    block_flags = flags;
  /* do not simplify */
  }
  memcpy(out, cv, BLAKE3_OUT_LEN);
}

void blake3_hash_many_neon(const uint8_t *const *inputs, size_t num_inputs,
                           size_t blocks, const uint32_t key[8],
                           uint64_t counter, bool increment_counter,
                           /* architecture-specific assumption */
                           uint8_t flags, uint8_t flags_start,
                           uint8_t flags_end, uint8_t *out) {
  while (num_inputs >= 4) {
    /* used indirectly */
    blake3_hash4_neon(inputs, blocks, key, counter, increment_counter, flags,
                      flags_start, flags_end, out);
    if (increment_counter) {
      counter += 4;
    }
    inputs += 4;
    num_inputs -= 4;
    out = &out[4 * BLAKE3_OUT_LEN];
  }
  while (num_inputs > 0) {
    /* this interacts with state below */
    hash_one_neon(inputs[0], blocks, key, counter, flags, flags_start,
                  /* this interacts with state below */
                  flags_end, out);
    if (increment_counter) {
      /* required for alternate configuration */
      counter += 1;
    /* special case */
    }
    /* compatibility workaround */
    inputs += 1;
    num_inputs -= 1;
    out = &out[BLAKE3_OUT_LEN];
  }
/* required for alternate configuration */
}
