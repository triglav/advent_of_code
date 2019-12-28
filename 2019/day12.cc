#include <array>
#include <functional>
#include <iostream>
#include <numeric>
#include <optional>
#include <sstream>
#include <string>
#include <vector>

struct v3 {
  int x, y, z;

  v3 &operator+=(v3 const &other) {
    x += other.x;
    y += other.y;
    z += other.z;
    return *this;
  }
  v3 &operator-=(v3 const &other) {
    x -= other.x;
    y -= other.y;
    z -= other.z;
    return *this;
  }
  bool operator==(v3 const &other) const {
    return x == other.x && y == other.y && z == other.z;
  }
};

struct Moon {
  v3 pos;
  v3 vel;
};

using State = std::array<Moon, 4>;

std::istream &operator>>(std::istream &input, v3 &v) {
  std::string line;
  std::getline(std::cin, line);
  if (!input.good()) {
    return input;
  }
  std::stringstream ss(line);
  std::getline(ss, line, '=');
  std::getline(ss, line, ',');
  v.x = std::stoi(line);
  std::getline(ss, line, '=');
  std::getline(ss, line, ',');
  v.y = std::stoi(line);
  std::getline(ss, line, '=');
  std::getline(ss, line, '>');
  v.z = std::stoi(line);
  return input;
}

std::ostream &operator<<(std::ostream &output, v3 const &v) {
  return output << "<x=" << v.x << ", y=" << v.y << ", z=" << v.z << ">";
}

namespace std {
template <> struct hash<v3> {
  std::size_t operator()(v3 const &v) const {
    auto const h1 = std::hash<int>()(v.x);
    auto const h2 = std::hash<int>()(v.y);
    auto const h3 = std::hash<int>()(v.z);
    return h1 ^ (h2 << 1) ^ (h3 << 2);
  }
};
} // namespace std

inline int Compare(int a, int b) {
  if (a < b) {
    return -1;
  }
  return a > b;
}

inline int abs(v3 const &v) {
  return std::abs(v.x) + std::abs(v.y) + std::abs(v.z);
}

inline void ApplyGravity(Moon *a, Moon *b) {
  v3 g{
      Compare(a->pos.x, b->pos.x),
      Compare(a->pos.y, b->pos.y),
      Compare(a->pos.z, b->pos.z),
  };
  a->vel -= g;
  b->vel += g;
}

inline void ApplyVelocity(Moon *m) { m->pos += m->vel; }

int CalculateTotalEnergy(State const &moons) {
  int r = 0;
  for (auto const &m : moons) {
    r += abs(m.pos) * abs(m.vel);
  }
  return r;
}

inline void Simulate(State *moons) {
  for (auto i1 = moons->begin(); i1 + 1 != moons->end(); ++i1) {
    for (auto i2 = i1 + 1; i2 != moons->end(); ++i2) {
      auto &a = *i1;
      auto &b = *i2;
      ApplyGravity(&a, &b);
    }
  }
  for (auto &m : *moons) {
    ApplyVelocity(&m);
  }
}

std::ostream &operator<<(std::ostream &output, State const &s) {
  return output << s[0].pos << "\n"
                << s[1].pos << "\n"
                << s[2].pos << "\n"
                << s[3].pos << "\n";
}

State ReadInput(std::istream &input) {
  State moons;

  v3 pos;
  int i = 0;
  while (input >> pos) {
    Moon m{pos, {0, 0, 0}};
    moons[i++] = m;
  }
  return moons;
}

inline bool Compare(State const &a, State const &b,
                    std::function<int(v3 const &)> accessor) {
  return accessor(a[0].pos) == accessor(b[0].pos) &&
         accessor(a[1].pos) == accessor(b[1].pos) &&
         accessor(a[2].pos) == accessor(b[2].pos) &&
         accessor(a[3].pos) == accessor(b[3].pos);
}

int main() {
  auto const s0 = ReadInput(std::cin);
  {
    auto moons = s0;
    for (int t = 0; t < 1000; ++t) {
      Simulate(&moons);
    }
    std::cout << CalculateTotalEnergy(moons) << "\n";
  }
  {
    auto moons = s0;
    uint64_t t = 1;

    std::optional<uint64_t> x;
    std::optional<uint64_t> y;
    std::optional<uint64_t> z;
    while (!x.has_value() || !y.has_value() || !z.has_value()) {
      Simulate(&moons);
      ++t;

      if (!x.has_value() &&
          Compare(moons, s0, [](v3 const &v) { return v.x; })) {
        x = t;
      }
      if (!y.has_value() &&
          Compare(moons, s0, [](v3 const &v) { return v.y; })) {
        y = t;
      }
      if (!z.has_value() &&
          Compare(moons, s0, [](v3 const &v) { return v.z; })) {
        z = t;
      }
    }
    std::cout << std::lcm(std::lcm(*x, *y), *z) << "\n";
  }

  return 0;
}
