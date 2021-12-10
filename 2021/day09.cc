#include <algorithm>
#include <iostream>
#include <string>
#include <unordered_set>
#include <vector>

#include "coord.h"

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

  void check(int x, int y, char l, std::vector<Coord> *todo) {
    if (x < 0 || x >= width || y < 0 || y >= height) {
      return;
    }
    auto l2 = loc(x, y);
    if (l2 >= l && l2 < 9) {
      todo->push_back({x, y});
    }
  }

  int ExploreBasin(int x, int y) {
    std::unordered_set<Coord> visited;
    std::vector<Coord> todo;
    todo.push_back({x, y});
    int size = 0;

    while (!todo.empty()) {
      auto const c = todo.back();
      todo.pop_back();

      if (visited.find(c) != visited.end()) {
        continue;
      }
      auto l = loc(c.x, c.y);
      size += 1;
      visited.insert(c);

      check(c.x - 1, c.y, l, &todo);
      check(c.x + 1, c.y, l, &todo);
      check(c.x, c.y - 1, l, &todo);
      check(c.x, c.y + 1, l, &todo);
    }
    return size;
  }
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

  std::vector<int> basin_sizes;
  int sum = 0;
  for (int y = 0; y < m.height; ++y) {
    for (int x = 0; x < m.width; ++x) {
      if (m.is_low_point(x, y)) {
        sum += m.loc(x, y) + 1;
        auto s = m.ExploreBasin(x, y);
        auto const it = std::lower_bound(basin_sizes.begin(), basin_sizes.end(),
                                         s, std::greater<int>());
        basin_sizes.insert(it, s);
      }
    }
  }
  std::cout << sum << "\n";
  std::cout << basin_sizes[0] * basin_sizes[1] * basin_sizes[2] << "\n";
  return 0;
}
