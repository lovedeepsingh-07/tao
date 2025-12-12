#include <emscripten/bind.h>
#include <emscripten/emscripten.h>

struct Color {
  uint8_t r, g, b, a;
};

extern "C" EMSCRIPTEN_KEEPALIVE Color get_bg_color() {
  // return Color{ 255, 141, 161, 255 };
  return Color{144, 213, 255, 255};
}

EMSCRIPTEN_BINDINGS(basement_module) {
  emscripten::value_object<Color>("Color")
      .field("r", &Color::r)
      .field("g", &Color::g)
      .field("b", &Color::b)
      .field("a", &Color::a);

  emscripten::function("get_bg_color", &get_bg_color);
}
