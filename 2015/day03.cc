#include <cassert>
#include <iostream>
#include <unordered_set>

#include "coord.h"

int main() {
  std::unordered_set<Coord> map;
  std::unordered_set<Coord> map2;

  Coord pos{0, 0};
  map.insert(pos);
  int houses = 1;

  std::array<Coord, 2> pos2{pos, pos};
  map2.insert(pos);
  int houses2 = 1;
  int i = 0;

  char c;
  while (std::cin >> c) {
    assert(c == '^' || c == '>' || c == 'v' || c == '<');
    if (c == '^') {
      --pos.y;
      --pos2[i].y;
    } else if (c == '>') {
      ++pos.x;
      ++pos2[i].x;
    } else if (c == 'v') {
      ++pos.y;
      ++pos2[i].y;
    } else if (c == '<') {
      --pos.x;
      --pos2[i].x;
    }

    if (map.find(pos) == map.end()) {
      ++houses;
      map.insert(pos);
    }
    if (map2.find(pos2[i]) == map2.end()) {
      ++houses2;
      map2.insert(pos2[i]);
    }
    i = (i + 1) % 2;
  }
  std::cout << houses << "\n" << houses2 << "\n";
  return 0;
}
