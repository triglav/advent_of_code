#include <cstdint>
#include <iostream>
#include <tuple>

constexpr uint64_t const first_value = 20151125;
constexpr uint64_t const a = 252533;
constexpr uint64_t const b = 33554393;

inline constexpr uint64_t next_value(uint64_t v) {
  return (v * a) % b;
}

constexpr std::tuple<int, int> next_coord(std::tuple<int, int> c) {
  auto const [x, y] = c;
  if (y > 1) {
    return std::make_tuple(x + 1, y - 1);
  }
  return std::make_tuple(1, x + 1);
}

std::ostream & operator<<(std::ostream & o, std::tuple<int, int> const & t) {
  auto const [x, y] = t;
  return o << x << "," << y;
}

int main() {
  int const row = 2947;
  int const column = 3029;

  auto pos = std::make_tuple(1, 1);
  auto val = first_value;
  while (true) {
    auto const [c, r] = pos;
    if (c == column && r == row) {
      std::cout << val << "\n";
      break;
    }
    pos = next_coord(pos);
    val = next_value(val);
  }

  return 0;
}
