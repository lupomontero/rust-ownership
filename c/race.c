#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <unistd.h>

volatile long int a = 0;

void threadFunc(void *arg) {
  int i;
  for (i = 1; i < 50000000; i++) {
    a = a + i;
  }
}

void threadFunc2(void *arg) {
  int i;
  for (i = 50000000; i <= 100000000; i++) {
    a = a + i;
  }
}

void main() {
  pthread_t one, two;

  pthread_create(&one, NULL, (void*)&threadFunc, NULL);
  pthread_create(&two, NULL, (void*)&threadFunc2, NULL);

  pthread_join(one, NULL);
  pthread_join(two, NULL);

  printf("%ld\n", a);
}
