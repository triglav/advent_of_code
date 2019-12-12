#include <iostream>
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
};

struct Moon {
  v3 pos;
  v3 vel;
};

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

int Compare(int a, int b) {
  if (a < b) {
    return -1;
  }
  return a > b;
}

int abs(v3 const &v) { return std::abs(v.x) + std::abs(v.y) + std::abs(v.z); }

void ApplyGravity(Moon *a, Moon *b) {
  v3 g{
      Compare(a->pos.x, b->pos.x),
      Compare(a->pos.y, b->pos.y),
      Compare(a->pos.z, b->pos.z),
  };
  a->vel -= g;
  b->vel += g;
}

inline void ApplyVelocity(Moon *m) { m->pos += m->vel; }

int CalculateTotalEnergy(std::vector<Moon> const &moons) {
  int r = 0;
  for (auto const &m : moons) {
    r += abs(m.pos) * abs(m.vel);
  }
  return r;
}

int main() {
  std::vector<Moon> moons;
  {
    v3 pos;
    while (std::cin >> pos) {
      Moon m{pos, {0, 0, 0}};
      moons.push_back(m);
    }
  }
  for (int t = 0; t < 1000; ++t) {
    for (auto i1 = moons.begin(); i1 + 1 != moons.end(); ++i1) {
      for (auto i2 = i1 + 1; i2 != moons.end(); ++i2) {
        auto &a = *i1;
        auto &b = *i2;
        ApplyGravity(&a, &b);
      }
    }
    for (auto &m : moons) {
      ApplyVelocity(&m);
    }
  }
  std::cout << CalculateTotalEnergy(moons) << "\n";

  return 0;
}
