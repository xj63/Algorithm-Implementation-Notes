#include <stdio.h>

void display_array(unsigned len, int array[len]) {
  if (len == 0) {
    printf("[]\n");
    return;
  }

  putchar('[');
  for (unsigned i = 0; i < len - 1; i++)
    printf("%d, ", array[i]);
  printf("%d]\n", array[len - 1]);
}

#define ARRAY_LEN(array) (sizeof(array) / sizeof(int))
#define DISPLAY_ARRAY(array) display_array(ARRAY_LEN(array), array)
