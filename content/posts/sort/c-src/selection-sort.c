#include "sort.h"

typedef struct MinMaxIndex {
  unsigned min;
  unsigned max;
} MinMaxIndex;

static inline MinMaxIndex select_maxmin_index(unsigned len, int array[len]) {
  MinMaxIndex minmax = {0, 0};

  for (unsigned i = 1; i < len; i++) {
    if (array[i] < array[minmax.min])
      minmax.min = i;
    else if (array[i] > array[minmax.max])
      minmax.max = i;
  }

  return minmax;
}

void selection_sort(unsigned len, int array[len]) {
  if (len < 2)
    return;

  MinMaxIndex minmax = select_maxmin_index(len, array);

  /* Error: Read the modified area
    // [4, 3, 3, 1] --error-> [4, 3, 3, 1]
    SWAP(array[minmax.min], array[0]);
    SWAP(array[minmax.max], array[len - 1]);
  */

  /* Error: Double write.
   * [3, 1, 2] --error-> [1, 3, 3]
    // copy
    int max = array[minmax.max];
    int min = array[minmax.min];
    int head = array[0];
    int tail = array[len - 1];
    // write
    array[minmax.min] = head;
    array[minmax.max] = tail;
    array[len - 1] = max;
    array[0] = min;
  */

  if (minmax.max == 0) {
    SWAP(array[minmax.min], array[0]);
    SWAP(array[minmax.min], array[len - 1]);
  } else {
    SWAP(array[minmax.min], array[0]);
    SWAP(array[minmax.max], array[len - 1]);
  }

  selection_sort(len - 2, &array[1]);
}

/* #define TEST */
#ifdef TEST

#include "test-utils.c"

void test_empty() {
  int array[] = {};
  selection_sort(0, array);
}

void test_sort() {
  /* int random[] = {1, 3, 5, 0, 3, -1, 4, 8}; */
  int random[] = {3, 1, 2};
  DISPLAY_ARRAY(random);
  selection_sort(ARRAY_LEN(random), random);
  DISPLAY_ARRAY(random);
}

int main() {
  test_empty();
  test_sort();
}

#endif /* ifdef TEST */
