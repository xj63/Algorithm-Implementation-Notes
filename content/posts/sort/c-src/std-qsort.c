#include <stdlib.h>

int compare(const void *a, const void *b) { return *(int *)a - *(int *)b; }

void std_qsort(unsigned len, int array[len]) {
  qsort(array, len, sizeof(int), compare);
}
