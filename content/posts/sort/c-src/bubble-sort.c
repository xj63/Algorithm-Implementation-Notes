#include "sort.h"

static inline void bubble_pass(unsigned len, int array[len]) {
  for (unsigned i = 1; i < len; i++)
    if (array[i - 1] > array[i])
      SWAP(array[i - 1], array[i]);
}

void bubble_sort(unsigned len, int array[len]) {
  for (unsigned i = len; i > 1; i--)
    bubble_pass(i, array);
}

/* #define TEST */
#ifdef TEST

#include "test-utils.c"

void test_empty() {
  int array[] = {};
  bubble_sort(0, array);
}

void test_sort() {
  int random[] = {1, 3, 5, 0, 3, -1, 4, 8};
  DISPLAY_ARRAY(random);
  bubble_sort(ARRAY_LEN(random), random);
  DISPLAY_ARRAY(random);
}

int main() {
  test_empty();
  test_sort();
}

#endif /* ifdef TEST */
