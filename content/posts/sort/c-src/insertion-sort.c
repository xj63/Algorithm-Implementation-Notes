#include "sort.h"

// user should ensure array is sorted.
static inline int insert_by_ord(unsigned len, int array[len], int element) {
  if (len == 0)
    return element;

  int last = array[len - 1];
  if (element >= last)
    // element is greater than or equal array max
    return element;

  // sliding window 2
  for (int i = len - 2; i >= 0; i--) {
    int wave = array[i];

    if (element >= wave) {
      // element meet the first less than or equal self
      array[i + 1] = element;
      return last;
    }

    array[i + 1] = wave;
  }

  // element is less than array min
  array[0] = element;
  return last;
}

void insertion_sort(unsigned len, int array[len]) {
  for (unsigned i = 0; i < len; i++) {
    int tmp = insert_by_ord(i, array, array[i]);
    array[i] = tmp;
  }
}

/* #define TEST */
#ifdef TEST

#include "test-utils.c"

void test_insert_array() {
  puts("test_insert_array:");
  int array[8] = {0};
  int tmp;
  printf("origin ");
  DISPLAY_ARRAY(array);
  int insert[] = {1, 3, 5, 0, 3, 4, 4, 8};
  printf("insert ");
  DISPLAY_ARRAY(insert);

  for (unsigned i = 0; i < ARRAY_LEN(insert); i++) {
    tmp = insert_by_ord(i, array, insert[i]);
    array[i] = tmp;
    DISPLAY_ARRAY(array);
  }
}

void test_insert_sort() {
  puts("test_insert_sort:");
  int array[] = {1, 3, 5, 0, 3, 4, 4, 8};
  DISPLAY_ARRAY(array);
  insertion_sort(ARRAY_LEN(array), array);
  DISPLAY_ARRAY(array);
}

/*
#include "random.h"
#include <stdlib.h>

#define COPY_ARRAY(DEST, SRC)                                                  \
  int DEST[ARRAY_LEN(SRC)];                                                    \
  for (unsigned i = 0; i < ARRAY_LEN(SRC); i++)                                \
    DEST[i] = SRC[i];

int compare(const void *a, const void *b) { return *(int *)a - *(int *)b; }

void test_insert_random() {
  COPY_ARRAY(insert_array, random_data);
  COPY_ARRAY(std_array, random_data);

  insertion_sort(ARRAY_LEN(random_data), insert_array);
  qsort(std_array, ARRAY_LEN(random_data), sizeof(int), compare);

  for (unsigned i = 0; i < ARRAY_LEN(random_data); i++)
    if (insert_array[i] != std_array[i])
      puts("test_insert_random failed");
}
*/

int main() {
  test_insert_array();
  putchar('\n');
  test_insert_sort();
  /* test_insert_random(); */
}

#endif /* ifdef TEST */
