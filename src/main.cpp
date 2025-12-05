#include <fmt/base.h>
#include <fmt/color.h>
#include <fmt/format.h>
#include <iostream>

int main() {
  fmt::println("{} {}", fmt::styled("[INFO]", fmt::fg(fmt::color::cyan)),
               fmt::format("closing fmt demo"));
  return 0;
}
