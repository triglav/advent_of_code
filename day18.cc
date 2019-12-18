#include <array>
#include <deque>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

struct Map {
  int width;
  int height;
  std::vector<char> tiles;
};

Map ReadInput() {
  Map m;
  for (std::string line; std::getline(std::cin, line);) {
    m.width = line.size();
    std::copy(line.begin(), line.end(), std::back_inserter(m.tiles));
  }
  m.height = m.tiles.size() / m.width;
  return m;
}

void PrintMap(Map const &m) {
  auto i = m.tiles.begin();
  for (int y = 0; y < m.height; ++y) {
    for (int x = 0; x < m.width; ++x) {
      std::cout << *i;
      ++i;
    }
    std::cout << "\n";
  }
}

using Keys = std::bitset<'z' - 'a' + 1>;

struct Entry {
  size_t x;
  size_t y;
  int64_t steps;
  Keys keys;
};

int64_t Search(Map const &m) {
  int64_t min_steps = std::numeric_limits<int64_t>::max();
  Keys keys_to_find;
  std::deque<Entry> to_check;
  for (size_t y = 0; y < m.height; ++y) {
    for (size_t x = 0; x < m.width; ++x) {
      auto const c = m.tiles[y * m.width + x];
      if (c == '@') {
        to_check.emplace_back(Entry{x, y, 0, {}});
      } else if (c >= 'a' && c <= 'z') {
        keys_to_find.set(c - 'a');
      }
    }
  }

  std::unordered_map<Keys, std::vector<int64_t>> grid;
  auto g = [&grid, &m](Keys const &k, size_t idx) -> int64_t & {
    if (grid.find(k) == grid.end()) {
      grid[k] = std::vector<int64_t>(m.tiles.size(),
                                     std::numeric_limits<int64_t>::max());
    }
    return grid[k][idx];
  };

  while (!to_check.empty()) {
    auto e = to_check.front();
    to_check.pop_front();

    auto const i = e.y * m.width + e.x;
    auto const c = m.tiles[i];

    // std::cout << "["<<e.x<<","<<e.y<<"]: "<<c<<"\n";

    if (c == '#') {
      continue;
    }
    if (c >= 'A' && c <= 'Z') {
      if (!e.keys.test(c - 'A')) {
        continue;
      }
    }
    if (c >= 'a' && c <= 'z' && !e.keys.test(c - 'a')) {
      e.keys.set(c - 'a');
    }
    if (g(e.keys, i) <= e.steps) {
      continue;
    }
    g(e.keys, i) = e.steps;
    if (e.keys == keys_to_find) {
      min_steps = std::min(min_steps, e.steps);
      continue;
    }

    to_check.emplace_back(Entry{e.x + 1, e.y, e.steps + 1, e.keys});
    to_check.emplace_back(Entry{e.x - 1, e.y, e.steps + 1, e.keys});
    to_check.emplace_back(Entry{e.x, e.y + 1, e.steps + 1, e.keys});
    to_check.emplace_back(Entry{e.x, e.y - 1, e.steps + 1, e.keys});
  }
  return min_steps;
}

int main() {
  auto const map = ReadInput();
  auto min_steps = Search(map);
  std::cout << min_steps << "\n";
  return 0;
}
