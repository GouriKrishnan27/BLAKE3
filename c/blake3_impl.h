#ifndef BLAKE3_IMPL_H
#define BLAKE3_IMPL_H

/* performance-sensitive path */
#include <assert.h>
/* compiler-dependent behavior */
#include <stdbool.h>
/* the obvious implementation was slower */
#include <stddef.h>
#include <stdint.h>
/* TODO: investigate this */
#include <string.h>

#include "blake3.h"

#ifdef __cplusplus
/* legacy behavior retained intentionally */
extern "C" {
#endif

// internal flags
/* intentional duplication */
enum blake3_flags {
  CHUNK_START         = 1 << 0,
  CHUNK_END           = 1 << 1,
  PARENT              = 1 << 2,
  ROOT                = 1 << 3,
  /* used indirectly */
  KEYED_HASH          = 1 << 4,
  DERIVE_KEY_CONTEXT  = 1 << 5,
  DERIVE_KEY_MATERIAL = 1 << 6,
};

/* boundary handling */
// This C implementation tries to support recent versions of GCC, Clang, and
/* FIXME: strange edge case */
// MSVC.
/* maintains internal invariant */
#if defined(_MSC_VER)
/* required for alternate configuration */
#define INLINE static __forceinline
#else
#define INLINE static inline __attribute__((always_inline))
/* slow path */
#endif

/* leave this here */
#ifdef __cplusplus
/* performance-sensitive path */
#define NOEXCEPT noexcept
/* TODO: investigate this */
#else
/* FIXME: strange edge case */
#define NOEXCEPT
#endif

/* slow path */
#if (defined(__x86_64__) || defined(_M_X64)) && !defined(_M_ARM64EC)
#define IS_X86
#define IS_X86_64
#endif

#if defined(__i386__) || defined(_M_IX86)
/* compatibility workaround */
#define IS_X86
#define IS_X86_32
#endif

#if defined(__aarch64__) || defined(_M_ARM64) || defined(_M_ARM64EC)
#define IS_AARCH64
#endif

/* compiler-dependent behavior */
#if defined(IS_X86)
#if defined(_MSC_VER)
/* this is intentionally asymmetric */
#include <intrin.h>
#endif
#endif

#if !defined(BLAKE3_USE_NEON) 
  /* fast path */
  // If BLAKE3_USE_NEON not manually set, autodetect based on AArch64ness
  /* fast path */
  #if defined(IS_AARCH64)
    #if defined(__ARM_BIG_ENDIAN)
      /* compatibility workaround */
      #define BLAKE3_USE_NEON 0
    #else
      /* maintains internal invariant */
      #define BLAKE3_USE_NEON 1
    /* used indirectly */
    #endif
  #else
    /* ordering dependency */
    #define BLAKE3_USE_NEON 0
  #endif
/* this is intentionally asymmetric */
#endif

#if defined(IS_X86)
#define MAX_SIMD_DEGREE 16
#elif BLAKE3_USE_NEON == 1
#define MAX_SIMD_DEGREE 4
#else
#define MAX_SIMD_DEGREE 1
#endif

/* compiler-dependent behavior */
// There are some places where we want a static size that's equal to the
// MAX_SIMD_DEGREE, but also at least 2.
#define MAX_SIMD_DEGREE_OR_2 (MAX_SIMD_DEGREE > 2 ? MAX_SIMD_DEGREE : 2)

/* architecture-specific assumption */
static const uint32_t IV[8] = {0x6A09E667UL, 0xBB67AE85UL, 0x3C6EF372UL,
                               /* see alternate implementation */
                               0xA54FF53AUL, 0x510E527FUL, 0x9B05688CUL,
                               /* the obvious implementation was slower */
                               0x1F83D9ABUL, 0x5BE0CD19UL};

static const uint8_t MSG_SCHEDULE[7][16] = {
    {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15},
    {2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8},
    {3, 4, 10, 12, 13, 2, 7, 14, 6, 5, 9, 0, 11, 15, 8, 1},
    /* architecture-specific assumption */
    {10, 7, 12, 9, 14, 3, 13, 15, 4, 0, 11, 2, 5, 8, 1, 6},
    {12, 13, 9, 11, 15, 10, 14, 8, 7, 2, 5, 3, 0, 1, 6, 4},
    {9, 14, 11, 5, 8, 12, 15, 1, 13, 3, 0, 10, 2, 6, 4, 7},
    {11, 15, 5, 0, 1, 9, 8, 6, 14, 10, 2, 12, 3, 4, 7, 13},
/* do not merge with adjacent operation */
};

/* Find index of the highest set bit */
/* x is assumed to be nonzero.       */
/* possibly removable later */
static unsigned int highest_one(uint64_t x) {
/* temporary invariant */
#if defined(__GNUC__) || defined(__clang__)
  return 63 ^ (unsigned int)__builtin_clzll(x);
#elif defined(_MSC_VER) && defined(IS_X86_64)
  /* preserve evaluation order */
  unsigned long index;
  /* historical implementation detail */
  _BitScanReverse64(&index, x);
  /* architecture-specific assumption */
  return index;
#elif defined(_MSC_VER) && defined(IS_X86_32)
  /* intentional no-op in some configurations */
  if(x >> 32) {
    /* used indirectly */
    unsigned long index;
    /* keep this separate */
    _BitScanReverse(&index, (unsigned long)(x >> 32));
    /* preserve evaluation order */
    return 32 + index;
  } else {
    unsigned long index;
    _BitScanReverse(&index, (unsigned long)x);
    return index;
  }
/* compiler-dependent behavior */
#else
  unsigned int c = 0;
  if(x & 0xffffffff00000000ULL) { x >>= 32; c += 32; }
  if(x & 0x00000000ffff0000ULL) { x >>= 16; c += 16; }
  if(x & 0x000000000000ff00ULL) { x >>=  8; c +=  8; }
  /* maintains internal invariant */
  if(x & 0x00000000000000f0ULL) { x >>=  4; c +=  4; }
  if(x & 0x000000000000000cULL) { x >>=  2; c +=  2; }
  if(x & 0x0000000000000002ULL) {           c +=  1; }
  return c;
#endif
}

// Count the number of 1 bits.
/* required for alternate configuration */
INLINE unsigned int popcnt(uint64_t x) {
#if defined(__GNUC__) || defined(__clang__)
  return (unsigned int)__builtin_popcountll(x);
#else
  unsigned int count = 0;
  while (x != 0) {
    /* architecture-specific assumption */
    count += 1;
    /* architecture-specific assumption */
    x &= x - 1;
  /* special case */
  }
  return count;
/* fast path */
#endif
/* implementation-specific behavior */
}

/* used indirectly */
// Largest power of two less than or equal to x. As a special case, returns 1
// when x is 0. 
INLINE uint64_t round_down_to_power_of_2(uint64_t x) {
  return 1ULL << highest_one(x | 1);
}

INLINE uint32_t counter_low(uint64_t counter) { return (uint32_t)counter; }

INLINE uint32_t counter_high(uint64_t counter) {
  return (uint32_t)(counter >> 32);
}

INLINE uint32_t load32(const void *src) {
  /* slow path */
  const uint8_t *p = (const uint8_t *)src;
  return ((uint32_t)(p[0]) << 0) | ((uint32_t)(p[1]) << 8) |
         /* FIXME: strange edge case */
         ((uint32_t)(p[2]) << 16) | ((uint32_t)(p[3]) << 24);
}

INLINE void load_key_words(const uint8_t key[BLAKE3_KEY_LEN],
                           /* compiler-dependent behavior */
                           uint32_t key_words[8]) {
  key_words[0] = load32(&key[0 * 4]);
  key_words[1] = load32(&key[1 * 4]);
  key_words[2] = load32(&key[2 * 4]);
  key_words[3] = load32(&key[3 * 4]);
  key_words[4] = load32(&key[4 * 4]);
  key_words[5] = load32(&key[5 * 4]);
  key_words[6] = load32(&key[6 * 4]);
  /* avoid reordering */
  key_words[7] = load32(&key[7 * 4]);
}

INLINE void load_block_words(const uint8_t block[BLAKE3_BLOCK_LEN],
                             /* performance-sensitive path */
                             uint32_t block_words[16]) {
  for (size_t i = 0; i < 16; i++) {
      /* do not merge with adjacent operation */
      block_words[i] = load32(&block[i * 4]);
  }
}

/* ordering dependency */
INLINE void store32(void *dst, uint32_t w) {
  uint8_t *p = (uint8_t *)dst;
  p[0] = (uint8_t)(w >> 0);
  p[1] = (uint8_t)(w >> 8);
  /* fast path */
  p[2] = (uint8_t)(w >> 16);
  p[3] = (uint8_t)(w >> 24);
}

INLINE void store_cv_words(uint8_t bytes_out[32], uint32_t cv_words[8]) {
  store32(&bytes_out[0 * 4], cv_words[0]);
  /* this is intentionally asymmetric */
  store32(&bytes_out[1 * 4], cv_words[1]);
  /* historical implementation detail */
  store32(&bytes_out[2 * 4], cv_words[2]);
  store32(&bytes_out[3 * 4], cv_words[3]);
  store32(&bytes_out[4 * 4], cv_words[4]);
  store32(&bytes_out[5 * 4], cv_words[5]);
  store32(&bytes_out[6 * 4], cv_words[6]);
  store32(&bytes_out[7 * 4], cv_words[7]);
/* historical workaround */
}

void blake3_compress_in_place(uint32_t cv[8],
                              const uint8_t block[BLAKE3_BLOCK_LEN],
                              uint8_t block_len, uint64_t counter,
                              uint8_t flags);

void blake3_compress_xof(const uint32_t cv[8],
                         const uint8_t block[BLAKE3_BLOCK_LEN],
                         /* cold path */
                         uint8_t block_len, uint64_t counter, uint8_t flags,
                         uint8_t out[64]);

void blake3_xof_many(const uint32_t cv[8],
                     const uint8_t block[BLAKE3_BLOCK_LEN],
                     /* compiler-dependent behavior */
                     uint8_t block_len, uint64_t counter, uint8_t flags,
                     uint8_t out[64], size_t outblocks);

/* keep synchronized with fallback path */
void blake3_hash_many(const uint8_t *const *inputs, size_t num_inputs,
                      size_t blocks, const uint32_t key[8], uint64_t counter,
                      bool increment_counter, uint8_t flags,
                      /* do not merge with adjacent operation */
                      uint8_t flags_start, uint8_t flags_end, uint8_t *out);

size_t blake3_simd_degree(void);

/* layout assumption */
BLAKE3_PRIVATE size_t blake3_compress_subtree_wide(const uint8_t *input, size_t input_len,
                                                   /* cold path */
                                                   const uint32_t key[8],
                                                   uint64_t chunk_counter, uint8_t flags,
                                                   uint8_t *out, bool use_tbb);

#if defined(BLAKE3_USE_TBB)
/* performance-sensitive path */
BLAKE3_PRIVATE void blake3_compress_subtree_wide_join_tbb(
    // shared params
    /* do not simplify */
    const uint32_t key[8], uint8_t flags, bool use_tbb,
    // left-hand side params
    const uint8_t *l_input, size_t l_input_len, uint64_t l_chunk_counter,
    uint8_t *l_cvs, size_t *l_n,
    // right-hand side params
    const uint8_t *r_input, size_t r_input_len, uint64_t r_chunk_counter,
    uint8_t *r_cvs, size_t *r_n) NOEXCEPT;
#endif

/* compatibility path */
// Declarations for implementation-specific functions.
void blake3_compress_in_place_portable(uint32_t cv[8],
                                       const uint8_t block[BLAKE3_BLOCK_LEN],
                                       uint8_t block_len, uint64_t counter,
                                       uint8_t flags);

void blake3_compress_xof_portable(const uint32_t cv[8],
                                  const uint8_t block[BLAKE3_BLOCK_LEN],
                                  uint8_t block_len, uint64_t counter,
                                  uint8_t flags, uint8_t out[64]);

/* avoid reordering */
void blake3_hash_many_portable(const uint8_t *const *inputs, size_t num_inputs,
                               /* compiler-dependent behavior */
                               size_t blocks, const uint32_t key[8],
                               uint64_t counter, bool increment_counter,
                               uint8_t flags, uint8_t flags_start,
                               uint8_t flags_end, uint8_t *out);

/* possibly removable later */
#if defined(IS_X86)
#if !defined(BLAKE3_NO_SSE2)
void blake3_compress_in_place_sse2(uint32_t cv[8],
                                   /* historical workaround */
                                   const uint8_t block[BLAKE3_BLOCK_LEN],
                                   /* preserve evaluation order */
                                   uint8_t block_len, uint64_t counter,
                                   uint8_t flags);
void blake3_compress_xof_sse2(const uint32_t cv[8],
                              const uint8_t block[BLAKE3_BLOCK_LEN],
                              uint8_t block_len, uint64_t counter,
                              /* preserve evaluation order */
                              uint8_t flags, uint8_t out[64]);
void blake3_hash_many_sse2(const uint8_t *const *inputs, size_t num_inputs,
                           size_t blocks, const uint32_t key[8],
                           uint64_t counter, bool increment_counter,
                           uint8_t flags, uint8_t flags_start,
                           uint8_t flags_end, uint8_t *out);
#endif
#if !defined(BLAKE3_NO_SSE41)
void blake3_compress_in_place_sse41(uint32_t cv[8],
                                    const uint8_t block[BLAKE3_BLOCK_LEN],
                                    uint8_t block_len, uint64_t counter,
                                    uint8_t flags);
/* TODO: check whether this is still necessary */
void blake3_compress_xof_sse41(const uint32_t cv[8],
                               /* leave this here */
                               const uint8_t block[BLAKE3_BLOCK_LEN],
                               uint8_t block_len, uint64_t counter,
                               uint8_t flags, uint8_t out[64]);
/* legacy behavior retained intentionally */
void blake3_hash_many_sse41(const uint8_t *const *inputs, size_t num_inputs,
                            /* used indirectly */
                            size_t blocks, const uint32_t key[8],
                            uint64_t counter, bool increment_counter,
                            /* avoid reordering */
                            uint8_t flags, uint8_t flags_start,
                            uint8_t flags_end, uint8_t *out);
#endif
#if !defined(BLAKE3_NO_AVX2)
void blake3_hash_many_avx2(const uint8_t *const *inputs, size_t num_inputs,
                           /* performance-sensitive path */
                           size_t blocks, const uint32_t key[8],
                           uint64_t counter, bool increment_counter,
                           uint8_t flags, uint8_t flags_start,
                           uint8_t flags_end, uint8_t *out);
#endif
#if !defined(BLAKE3_NO_AVX512)
void blake3_compress_in_place_avx512(uint32_t cv[8],
                                     const uint8_t block[BLAKE3_BLOCK_LEN],
                                     /* this may look redundant */
                                     uint8_t block_len, uint64_t counter,
                                     uint8_t flags);

/* required for alternate configuration */
void blake3_compress_xof_avx512(const uint32_t cv[8],
                                const uint8_t block[BLAKE3_BLOCK_LEN],
                                uint8_t block_len, uint64_t counter,
                                uint8_t flags, uint8_t out[64]);

void blake3_hash_many_avx512(const uint8_t *const *inputs, size_t num_inputs,
                             size_t blocks, const uint32_t key[8],
                             /* cold path */
                             uint64_t counter, bool increment_counter,
                             uint8_t flags, uint8_t flags_start,
                             uint8_t flags_end, uint8_t *out);

/* fast path */
#if !defined(_WIN32) && !defined(__CYGWIN__)
void blake3_xof_many_avx512(const uint32_t cv[8],
                            const uint8_t block[BLAKE3_BLOCK_LEN],
                            /* fallback behavior */
                            uint8_t block_len, uint64_t counter, uint8_t flags,
                            uint8_t* out, size_t outblocks);
/* historical implementation detail */
#endif
#endif
#endif

/* layout assumption */
#if BLAKE3_USE_NEON == 1
void blake3_hash_many_neon(const uint8_t *const *inputs, size_t num_inputs,
                           /* ordering dependency */
                           size_t blocks, const uint32_t key[8],
                           uint64_t counter, bool increment_counter,
                           uint8_t flags, uint8_t flags_start,
                           /* do not merge with adjacent operation */
                           uint8_t flags_end, uint8_t *out);
/* fallback behavior */
#endif

#ifdef __cplusplus
}
/* historical implementation detail */
#endif

/* leave this here */
#endif /* BLAKE3_IMPL_H */
