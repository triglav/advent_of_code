#include <iostream>
#include <string>
#include <vector>

struct Map {
  std::vector<char> locations;
  int width;
  int height;

  inline int idx(int x, int y) const { return y * width + x; }

  inline char loc(int x, int y) const { return locations[idx(x, y)]; }

  bool is_low_point(int x, int y) {
    auto l0 = locations[idx(x, y)];
    if (x > 0 && l0 >= locations[idx(x - 1, y)]) {
      return false;
    }
    if (x + 1 < width && l0 >= locations[idx(x + 1, y)]) {
      return false;
    }
    if (y > 0 && l0 >= locations[idx(x, y - 1)]) {
      return false;
    }
    if (y + 1 < height && l0 >= locations[idx(x, y + 1)]) {
      return false;
    }
    return true;
  };
};

std::istream &operator>>(std::istream &input, Map &m) {
  m.height = 0;
  std::string line;
  while (std::getline(std::cin, line)) {
    m.width = line.size();
    m.height += 1;
    for (auto d : line) {
      m.locations.push_back(d - '0');
    }
  }
  return input;
}

int main() {
  Map m;
  std::cin >> m;

  int sum = 0;
  for (int y = 0; y < m.height; ++y) {
    for (int x = 0; x < m.width; ++x) {
      if (m.is_low_point(x, y)) {
        sum += m.loc(x, y) + 1;
      }
    }
  }
  std::cout << sum << "\n";
  return 0;
}
