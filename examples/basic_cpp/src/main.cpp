#include "app_utils.hpp"
#include "context.hpp"
#include <cstdio>

int main() {
  printf("hello from main.cpp\n");
  app_utils::hello();
  context::hello();
  return 0;
}
