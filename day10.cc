#include <iostream>
#include <vector>

using Map = std::vector<bool>;
int width = 0;
int height = 0;

inline int index(int x, int y) { return y * width + x; }

bool ReadMapRow(Map *map) {
  std::string row;
  if (!std::getline(std::cin, row)) {
    return false;
  }
  for (auto c : row) {
    map->push_back(c == '#');
  }
  return true;
}

int Check(Map const &map, int const ax, int const ay) {
  Map hit_map(width * height, false);
  hit_map[index(ax, ay)] = true;
  int asteroid_count = 0;
  for (int i = 1; i < height + width; ++i) {
    auto const min_y = std::max(0, ay - i);
    auto const max_y = std::min(height, ay + i);
    for (int y = min_y; y < max_y; ++y) {
      auto const min_x = std::max(0, ax - i);
      auto const max_x = std::min(width, ax + i);
      for (int x = min_x; x < max_x; ++x) {
        if (hit_map[index(x, y)]) {
          continue;
        }

        auto const dx = x - ax;
        auto const dy = y - ay;

        bool shadow = false;
        int k = 1;
        while (true) {
          auto const x2 = ax + dx * k;
          auto const y2 = ay + dy * k;
          if (x2 < 0 || x2 >= width || y2 < 0 || y2 >= height) {
            break;
          }
          auto const idx = index(x2, y2);
          hit_map[idx] = true;
          if (!shadow && map[idx]) {
            ++asteroid_count;
            shadow = true;
          }
          ++k;
        }
      }
    }
  }
  return asteroid_count;
}

int main() {
  Map map;

  ReadMapRow(&map);
  width = map.size();
  while (ReadMapRow(&map)) {
    // nope
  }
  height = map.size() / width;

  int max_asteroids = 0;
  for (int y = 0; y < height; ++y) {
    for (int x = 0; x < width; ++x) {
      if (!map[index(x, y)]) {
        continue;
      }
      auto const z = Check(map, x, y);
      max_asteroids = std::max(max_asteroids, z);
    }
  }
  std::cout << max_asteroids << "\n";
  return 0;
}
