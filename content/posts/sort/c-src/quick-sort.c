#include "sort.h"

void quick_sort(unsigned len, int array[len]) {
  if (len <= 1)
    return;

  int pivot = array[0]; // take first as pivot
  int *left = &array[1];
  int *right = &array[len];

  while (left < right) {
    if (*left <= pivot) {
      left += 1;
    } else {
      right -= 1;
      SWAP(*left, *right);
    }
  }

  SWAP(*(left - 1), array[0]);

  quick_sort(left - array - 1, array);
  quick_sort(array + len - right, right);
}

/* #define TEST */
#ifdef TEST

#include "test-utils.c"

void test_bias() {
  int array[] = {1, 3, 5, 0, 3, 4, 4, 8};
  display_array(8, array);
  quick_sort(8, array);
  // array = {0, 1, 3, 3, 4, 4, 5, 8}
  display_array(8, array);
}

void test_rev() {
  int array[] = {5, 4, 3, 2, 1};
  DISPLAY_ARRAY(array);
  quick_sort(5, array);
  DISPLAY_ARRAY(array);
}

int main(int argc, char *argv[]) { test_bias(); }

#endif /* ifdef TEST */
