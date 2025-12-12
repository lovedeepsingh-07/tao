#include "app_utils.h"
#include "context.h"
#include <stdio.h>

int main() {
  printf("hello from main.c\n");
  utils_hello();
  context_hello();
  return 0;
}
