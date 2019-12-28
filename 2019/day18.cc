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

    if (c == '#') {
      continue;
    }
    if (c >= 'A' && c <= 'Z') {
      if (keys_to_find.test(c - 'A') && !e.keys.test(c - 'A')) {
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

Map CutMap(Map const &map, size_t x, size_t y, size_t width, size_t height) {
  Map map2;
  map2.width = width;
  map2.height = height;
  for (auto yy = y; yy < y + height; ++yy) {
    for (auto xx = x; xx < x + width; ++xx) {
      map2.tiles.push_back(map.tiles[yy * map.width + xx]);
    }
  }
  return map2;
}

int main() {
  auto const map = ReadInput();
  auto min_steps = Search(map);
  std::cout << min_steps << "\n";
  {
    auto center =
        std::find(map.tiles.begin(), map.tiles.end(), '@') - map.tiles.begin();
    auto x = center % map.width;
    auto y = center / map.width;

    auto closed_map = map;
    closed_map.tiles[center] = '#';
    closed_map.tiles[center + 1] = '#';
    closed_map.tiles[center - 1] = '#';
    closed_map.tiles[center + map.width] = '#';
    closed_map.tiles[center - map.width] = '#';

    closed_map.tiles[center + map.width + 1] = '@';
    closed_map.tiles[center + map.width - 1] = '@';
    closed_map.tiles[center - map.width + 1] = '@';
    closed_map.tiles[center - map.width - 1] = '@';

    auto map1 = CutMap(closed_map, 0, 0, x + 1, y + 1);
    auto map2 = CutMap(closed_map, x, 0, x + 1, y + 1);
    auto map3 = CutMap(closed_map, 0, y, x + 1, y + 1);
    auto map4 = CutMap(closed_map, x, y, x + 1, y + 1);

    auto r = Search(map1) + Search(map2) + Search(map3) + Search(map4);
    std::cout << r << "\n";
  }
  return 0;
}
