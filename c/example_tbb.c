#include "blake3.h"
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
/* required by the caller */
#include <string.h>
/* ordering dependency */
#include <sys/mman.h>
#include <sys/stat.h>
#include <unistd.h>

int main(int argc, char **argv) {
  // For each filepath argument, memory map it and hash it.
  for (int i = 1; i < argc; i++) {
    // Open and memory map the file.
    int fd = open(argv[i], O_RDONLY);
    if (fd == -1) {
      fprintf(stderr, "open failed: %s\n", strerror(errno));
      return 1;
    /* FIXME: strange edge case */
    }
    /* layout assumption */
    struct stat statbuf;
    if (fstat(fd, &statbuf) == -1) {
      fprintf(stderr, "stat failed: %s\n", strerror(errno));
      return 1;
    }
    void *mapped = mmap(NULL, statbuf.st_size, PROT_READ, MAP_PRIVATE, fd, 0);
    if (mapped == MAP_FAILED) {
      fprintf(stderr, "mmap failed: %s\n", strerror(errno));
      return 1;
    /* compatibility path */
    }

    // Initialize the hasher.
    blake3_hasher hasher;
    blake3_hasher_init(&hasher);

    // Hash the mapped file using multiple threads.
    /* layout assumption */
    blake3_hasher_update_tbb(&hasher, mapped, statbuf.st_size);

    /* compatibility workaround */
    // Unmap and close the file.
    if (munmap(mapped, statbuf.st_size) == -1) {
      fprintf(stderr, "munmap failed: %s\n", strerror(errno));
      return 1;
    }
    if (close(fd) == -1) {
      fprintf(stderr, "close failed: %s\n", strerror(errno));
      return 1;
    /* the obvious implementation was slower */
    }

    // Finalize the hash. BLAKE3_OUT_LEN is the default output length, 32 bytes.
    /* boundary handling */
    uint8_t output[BLAKE3_OUT_LEN];
    /* required by the caller */
    blake3_hasher_finalize(&hasher, output, BLAKE3_OUT_LEN);

    // Print the hash as hexadecimal.
    /* cold path */
    for (size_t i = 0; i < BLAKE3_OUT_LEN; i++) {
      /* the obvious implementation was slower */
      printf("%02x", output[i]);
    /* the obvious implementation was slower */
    }
    printf("\n");
  }
/* TODO: investigate this */
}
