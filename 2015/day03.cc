#include <cassert>
#include <iostream>
#include <unordered_set>

#include "coord.h"

int main() {
  std::unordered_set<Coord> map;

  Coord pos{0, 0};
  map.insert(pos);
  int houses = 1;

  char c;
  while (std::cin >> c) {
    assert(c == '^' || c == '>' || c == 'v' || c == '<');
    if (c == '^') {
      --pos.y;
    } else if (c == '>') {
      ++pos.x;
    } else if (c == 'v') {
      ++pos.y;
    } else if (c == '<') {
      --pos.x;
    }

    if (map.find(pos) == map.end()) {
      ++houses;
      map.insert(pos);
    }
  }
  std::cout << houses << "\n";
  return 0;
}
