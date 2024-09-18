#include "sort.h"
#include <stdbool.h>

void merge_sort_rec(unsigned len, int array[len]) {
  if (len <= 1)
    return;

  unsigned half = len / 2;
  merge_sort_rec(half, array);
  merge_sort_rec(len - half, &array[half]);

  merge_two_sorted_array(len, array, half);
}

// return block_size
unsigned parallel_sort_blocks(unsigned len, int array[len]) {
  const unsigned DEFAULT_BLOCK_SIZE = 128;
  const unsigned MAX_BLOCKS = 64;
  unsigned block_size = len / DEFAULT_BLOCK_SIZE > MAX_BLOCKS
                            ? len / MAX_BLOCKS
                            : DEFAULT_BLOCK_SIZE;
  unsigned blocks = len / block_size;

#pragma omp parallel for
  for (unsigned i = 0; i < blocks; i++)
    merge_sort_rec(block_size, &array[i * block_size]);

  if (len > block_size * blocks)
    // the last block is not full.
    merge_sort_rec(len - (blocks * block_size), &array[blocks * block_size]);

  return block_size;
}

void merge_adjacent_blocks(unsigned len, int array[len], unsigned block_size) {
  if (block_size >= len)
    // Only have one block and have sorted.
    return;

  unsigned blocks = len / block_size; // blocks >= 1
  for (unsigned i = 0; i < blocks / 2; i++)
    merge_two_sorted_array(block_size * 2, &array[i * 2 * block_size],
                           block_size);

  if (blocks % 2 == 1 && len > block_size * blocks) {
    // The last block is not full and remain the second last block not merge.
    unsigned lave = len - block_size * blocks;
    merge_two_sorted_array(block_size + lave, &array[len - lave - block_size],
                           block_size);
  }
}

void merge_sort_parallel(unsigned len, int array[len]) {
  unsigned block_size = parallel_sort_blocks(len, array);
  for (; block_size <= len; block_size *= 2)
    merge_adjacent_blocks(len, array, block_size);
}

void merge_sort_adjacent_blocks(unsigned len, int array[len]) {
  for (unsigned block_size = 1; block_size <= len; block_size *= 2)
    merge_adjacent_blocks(len, array, block_size);
}

void merge_sort(unsigned len, int array[len]) { merge_sort_rec(len, array); }

/* #define TEST */
#ifdef TEST

#include "test-utils.c"

#define TEST_FN(Ident, ...)                                                    \
  void test_##Ident() {                                                        \
    puts(#Ident ":");                                                          \
    int array[] = __VA_ARGS__;                                                 \
    DISPLAY_ARRAY(array);                                                      \
    merge_sort(ARRAY_LEN(array), array);                                       \
    DISPLAY_ARRAY(array);                                                      \
  }

#define CALL_TEST_FN(Ident)                                                    \
  test_##Ident();                                                              \
  putchar('\n');

TEST_FN(merge_sort_empty, {});
TEST_FN(merge_sort_one, {1});
TEST_FN(merge_sort_two, {1, 0});
TEST_FN(merge_sort_three, {1, 2, 0});
TEST_FN(merge_sort_four, {1, 2, 0, 3});
TEST_FN(merge_sort_five, {1, 2, 0, 3, -1});
TEST_FN(merge_sort_some, {1, 3, 5, 0, 3, -1, 4, 8});

int main() {
  CALL_TEST_FN(merge_sort_empty);
  CALL_TEST_FN(merge_sort_one);
  CALL_TEST_FN(merge_sort_two);
  CALL_TEST_FN(merge_sort_three);
  CALL_TEST_FN(merge_sort_four);
  CALL_TEST_FN(merge_sort_five);
  CALL_TEST_FN(merge_sort_some);
}

#endif /* ifdef TEST */
