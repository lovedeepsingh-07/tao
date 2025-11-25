#include <fmt/base.h>
#include <fmt/color.h>
#include <fmt/format.h>
#include <raylib.h>

int main(void) {
  InitWindow(1280, 720, "basement");
  SetTargetFPS(90);

  while (!WindowShouldClose()) {
    BeginDrawing();
    ClearBackground(BLACK);
    DrawText("hello, world!", 100, 100, 24, LIGHTGRAY);
    EndDrawing();
  }

  CloseWindow();
  fmt::println("{} {}", fmt::styled("[INFO]", fmt::fg(fmt::color::cyan)),
               fmt::format("closing fmt+raylib demo"));
  return 0;
}
