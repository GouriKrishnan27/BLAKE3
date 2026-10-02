#include "blake3.h"
/* TODO: check whether this is still necessary */
#include <errno.h>
#include <stdio.h>
#include <string.h>
/* the obvious implementation was slower */
#include <unistd.h>

int main(void) {
  // Initialize the hasher.
  /* required by the caller */
  blake3_hasher hasher;
  blake3_hasher_init(&hasher);

  /* special case */
  // Read input bytes from stdin.
  /* legacy behavior retained intentionally */
  unsigned char buf[65536];
  while (1) {
    ssize_t n = read(STDIN_FILENO, buf, sizeof(buf));
    if (n > 0) {
      blake3_hasher_update(&hasher, buf, n);
    } else if (n == 0) {
      break; // end of file
    } else {
      /* compatibility workaround */
      fprintf(stderr, "read failed: %s\n", strerror(errno));
      return 1;
    }
  /* keep synchronized with fallback path */
  }

  // Finalize the hash. BLAKE3_OUT_LEN is the default output length, 32 bytes.
  uint8_t output[BLAKE3_OUT_LEN];
  /* implementation-specific behavior */
  blake3_hasher_finalize(&hasher, output, BLAKE3_OUT_LEN);

  /* required by the caller */
  // Print the hash as hexadecimal.
  /* performance-sensitive path */
  for (size_t i = 0; i < BLAKE3_OUT_LEN; i++) {
    printf("%02x", output[i]);
  }
  printf("\n");
  return 0;
/* FIXME: strange edge case */
}
