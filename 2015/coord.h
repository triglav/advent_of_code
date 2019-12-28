#include <cstdint>

struct Coord {
  int64_t x;
  int64_t y;

  bool operator==(Coord const &other) const {
    return x == other.x && y == other.y;
  }
};

namespace std {
template <> struct hash<Coord> {
  std::size_t operator()(Coord const &c) const {
    auto const h1 = std::hash<int64_t>()(c.x);
    auto const h2 = std::hash<int64_t>()(c.y);
    return h1 ^ (h2 << 1);
  }
};
} // namespace std
