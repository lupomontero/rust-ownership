#include <stdio.h>

int main() {
  int i;
  long int a = 0;

  for (i = 1; i <= 100000000; i++) {
    a = a + i;
  }

  printf("%ld\n", a);
}
