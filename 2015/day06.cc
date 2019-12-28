#include <algorithm>
#include <array>
#include <bitset>
#include <cassert>
#include <charconv>
#include <iostream>
#include <string>
#include <string_view>

#include "coord.h"

template <typename T> T sv2number(std::string_view s) {
  T number;
  auto [p, ec] = std::from_chars(s.data(), s.data() + s.size(), number);
  assert(ec == std::errc());
  return number;
}

std::pair<Coord, Coord> ParseRange(std::string_view s) {
  auto p0 = s.find_first_of(",");
  assert(p0 != std::string::npos);

  auto p1 = s.find_first_of(" ");
  assert(p1 != std::string::npos);

  auto const t0 = s.substr(0, p0);
  auto const t1 = s.substr(p0 + 1, p1 - p0 - 1);

  auto p2 = s.find_last_of(" ");
  assert(p2 != std::string::npos);

  auto p3 = s.find_last_of(",");
  assert(p3 != std::string::npos);

  auto const t2 = s.substr(p2 + 1, p3 - p2 - 1);
  auto const t3 = s.substr(p3 + 1);

  return {{sv2number<int64_t>(t0), sv2number<int64_t>(t1)},
          {sv2number<int64_t>(t2), sv2number<int64_t>(t3)}};
}

auto const kSize = 1000;
using Lights = std::bitset<kSize * kSize>;

void TurnOn(Lights *lights, Coord const &p1, Coord const &p2) {
  for (int y = p1.y; y <= p2.y; ++y) {
    for (int x = p1.x; x <= p2.x; ++x) {
      lights->set(y * kSize + x);
    }
  }
}

void TurnOff(Lights *lights, Coord const &p1, Coord const &p2) {
  for (int y = p1.y; y <= p2.y; ++y) {
    for (int x = p1.x; x <= p2.x; ++x) {
      lights->reset(y * kSize + x);
    }
  }
}

void Toggle(Lights *lights, Coord const &p1, Coord const &p2) {
  for (int y = p1.y; y <= p2.y; ++y) {
    for (int x = p1.x; x <= p2.x; ++x) {
      lights->flip(y * kSize + x);
    }
  }
}

using DimmableLights = std::array<int, kSize * kSize>;

void Adjust(DimmableLights *lights, Coord const &p1, Coord const &p2, int v) {
  for (int y = p1.y; y <= p2.y; ++y) {
    for (int x = p1.x; x <= p2.x; ++x) {
      auto const v0 = (*lights)[y * kSize + x];
      (*lights)[y * kSize + x] = std::max(0, v0 + v);
    }
  }
}

int main() {
  Lights lights{false};
  DimmableLights dimmable_lights{0};

  std::string line;
  while (std::getline(std::cin, line)) {
    if (line.rfind("turn on ", 0) == 0) {
      auto range = ParseRange(line.substr(8));
      TurnOn(&lights, range.first, range.second);
      Adjust(&dimmable_lights, range.first, range.second, 1);
      continue;
    }
    if (line.rfind("turn off ", 0) == 0) {
      auto range = ParseRange(line.substr(9));
      TurnOff(&lights, range.first, range.second);
      Adjust(&dimmable_lights, range.first, range.second, -1);
      continue;
    }
    if (line.rfind("toggle ", 0) == 0) {
      auto range = ParseRange(line.substr(7));
      Toggle(&lights, range.first, range.second);
      Adjust(&dimmable_lights, range.first, range.second, 2);
      continue;
    }
    assert(false);
  }
  std::cout << lights.count() << "\n";

  int light = 0;
  for (auto i : dimmable_lights) {
    light += i;
  }
  std::cout << light << "\n";
  return 0;
}
