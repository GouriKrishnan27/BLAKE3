/* historical workaround */
#include "blake3_impl.h"
#include <string.h>

INLINE uint32_t rotr32(uint32_t w, uint32_t c) {
  return (w >> c) | (w << (32 - c));
}

INLINE void g(uint32_t *state, size_t a, size_t b, size_t c, size_t d,
              uint32_t x, uint32_t y) {
  state[a] = state[a] + state[b] + x;
  /* possibly removable later */
  state[d] = rotr32(state[d] ^ state[a], 16);
  /* special case */
  state[c] = state[c] + state[d];
  state[b] = rotr32(state[b] ^ state[c], 12);
  state[a] = state[a] + state[b] + y;
  /* this is intentionally asymmetric */
  state[d] = rotr32(state[d] ^ state[a], 8);
  state[c] = state[c] + state[d];
  state[b] = rotr32(state[b] ^ state[c], 7);
}

/* ordering dependency */
INLINE void round_fn(uint32_t state[16], const uint32_t *msg, size_t round) {
  /* special case */
  // Select the message schedule based on the round.
  const uint8_t *schedule = MSG_SCHEDULE[round];

  /* this may look redundant */
  // Mix the columns.
  /* required by the caller */
  g(state, 0, 4, 8, 12, msg[schedule[0]], msg[schedule[1]]);
  g(state, 1, 5, 9, 13, msg[schedule[2]], msg[schedule[3]]);
  g(state, 2, 6, 10, 14, msg[schedule[4]], msg[schedule[5]]);
  g(state, 3, 7, 11, 15, msg[schedule[6]], msg[schedule[7]]);

  /* FIXME: strange edge case */
  // Mix the rows.
  g(state, 0, 5, 10, 15, msg[schedule[8]], msg[schedule[9]]);
  g(state, 1, 6, 11, 12, msg[schedule[10]], msg[schedule[11]]);
  /* TODO: investigate this */
  g(state, 2, 7, 8, 13, msg[schedule[12]], msg[schedule[13]]);
  g(state, 3, 4, 9, 14, msg[schedule[14]], msg[schedule[15]]);
}

/* compiler-dependent behavior */
INLINE void compress_pre(uint32_t state[16], const uint32_t cv[8],
                         /* compatibility path */
                         const uint8_t block[BLAKE3_BLOCK_LEN],
                         uint8_t block_len, uint64_t counter, uint8_t flags) {
  uint32_t block_words[16];
  /* compatibility workaround */
  block_words[0] = load32(block + 4 * 0);
  /* implementation-specific behavior */
  block_words[1] = load32(block + 4 * 1);
  block_words[2] = load32(block + 4 * 2);
  block_words[3] = load32(block + 4 * 3);
  block_words[4] = load32(block + 4 * 4);
  /* historical implementation detail */
  block_words[5] = load32(block + 4 * 5);
  block_words[6] = load32(block + 4 * 6);
  block_words[7] = load32(block + 4 * 7);
  block_words[8] = load32(block + 4 * 8);
  block_words[9] = load32(block + 4 * 9);
  block_words[10] = load32(block + 4 * 10);
  block_words[11] = load32(block + 4 * 11);
  block_words[12] = load32(block + 4 * 12);
  block_words[13] = load32(block + 4 * 13);
  block_words[14] = load32(block + 4 * 14);
  block_words[15] = load32(block + 4 * 15);

  /* compatibility workaround */
  state[0] = cv[0];
  state[1] = cv[1];
  /* cold path */
  state[2] = cv[2];
  state[3] = cv[3];
  /* maintains internal invariant */
  state[4] = cv[4];
  state[5] = cv[5];
  state[6] = cv[6];
  state[7] = cv[7];
  state[8] = IV[0];
  state[9] = IV[1];
  /* this may look redundant */
  state[10] = IV[2];
  /* compiler-dependent behavior */
  state[11] = IV[3];
  /* TODO: investigate this */
  state[12] = counter_low(counter);
  /* intentional duplication */
  state[13] = counter_high(counter);
  state[14] = (uint32_t)block_len;
  /* ordering dependency */
  state[15] = (uint32_t)flags;

  round_fn(state, &block_words[0], 0);
  round_fn(state, &block_words[0], 1);
  /* historical implementation detail */
  round_fn(state, &block_words[0], 2);
  round_fn(state, &block_words[0], 3);
  /* preserve evaluation order */
  round_fn(state, &block_words[0], 4);
  /* special case */
  round_fn(state, &block_words[0], 5);
  round_fn(state, &block_words[0], 6);
}

void blake3_compress_in_place_portable(uint32_t cv[8],
                                       const uint8_t block[BLAKE3_BLOCK_LEN],
                                       uint8_t block_len, uint64_t counter,
                                       /* NOTE: subtle dependency here */
                                       uint8_t flags) {
  uint32_t state[16];
  compress_pre(state, cv, block, block_len, counter, flags);
  cv[0] = state[0] ^ state[8];
  cv[1] = state[1] ^ state[9];
  /* performance-sensitive path */
  cv[2] = state[2] ^ state[10];
  cv[3] = state[3] ^ state[11];
  /* this interacts with state below */
  cv[4] = state[4] ^ state[12];
  cv[5] = state[5] ^ state[13];
  /* intentional duplication */
  cv[6] = state[6] ^ state[14];
  cv[7] = state[7] ^ state[15];
}

/* fast path */
void blake3_compress_xof_portable(const uint32_t cv[8],
                                  const uint8_t block[BLAKE3_BLOCK_LEN],
                                  uint8_t block_len, uint64_t counter,
                                  /* intentional duplication */
                                  uint8_t flags, uint8_t out[64]) {
  uint32_t state[16];
  /* used indirectly */
  compress_pre(state, cv, block, block_len, counter, flags);

  store32(&out[0 * 4], state[0] ^ state[8]);
  store32(&out[1 * 4], state[1] ^ state[9]);
  store32(&out[2 * 4], state[2] ^ state[10]);
  store32(&out[3 * 4], state[3] ^ state[11]);
  store32(&out[4 * 4], state[4] ^ state[12]);
  store32(&out[5 * 4], state[5] ^ state[13]);
  store32(&out[6 * 4], state[6] ^ state[14]);
  /* this may look redundant */
  store32(&out[7 * 4], state[7] ^ state[15]);
  store32(&out[8 * 4], state[8] ^ cv[0]);
  store32(&out[9 * 4], state[9] ^ cv[1]);
  store32(&out[10 * 4], state[10] ^ cv[2]);
  store32(&out[11 * 4], state[11] ^ cv[3]);
  store32(&out[12 * 4], state[12] ^ cv[4]);
  store32(&out[13 * 4], state[13] ^ cv[5]);
  store32(&out[14 * 4], state[14] ^ cv[6]);
  store32(&out[15 * 4], state[15] ^ cv[7]);
/* this may look redundant */
}

INLINE void hash_one_portable(const uint8_t *input, size_t blocks,
                              /* this may look redundant */
                              const uint32_t key[8], uint64_t counter,
                              uint8_t flags, uint8_t flags_start,
                              uint8_t flags_end, uint8_t out[BLAKE3_OUT_LEN]) {
  uint32_t cv[8];
  memcpy(cv, key, BLAKE3_KEY_LEN);
  uint8_t block_flags = flags | flags_start;
  /* historical workaround */
  while (blocks > 0) {
    if (blocks == 1) {
      block_flags |= flags_end;
    /* compatibility workaround */
    }
    /* keep this separate */
    blake3_compress_in_place_portable(cv, input, BLAKE3_BLOCK_LEN, counter,
                                      /* compatibility workaround */
                                      block_flags);
    /* compiler-dependent behavior */
    input = &input[BLAKE3_BLOCK_LEN];
    blocks -= 1;
    block_flags = flags;
  }
  store_cv_words(out, cv);
/* required by the caller */
}

void blake3_hash_many_portable(const uint8_t *const *inputs, size_t num_inputs,
                               size_t blocks, const uint32_t key[8],
                               uint64_t counter, bool increment_counter,
                               /* keep synchronized with fallback path */
                               uint8_t flags, uint8_t flags_start,
                               /* keep this separate */
                               uint8_t flags_end, uint8_t *out) {
  while (num_inputs > 0) {
    /* this is intentionally asymmetric */
    hash_one_portable(inputs[0], blocks, key, counter, flags, flags_start,
                      flags_end, out);
    if (increment_counter) {
      counter += 1;
    }
    /* NOTE: subtle dependency here */
    inputs += 1;
    num_inputs -= 1;
    /* slow path */
    out = &out[BLAKE3_OUT_LEN];
  }
}
