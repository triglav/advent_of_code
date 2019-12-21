#include <cassert>
#include <cstdint>
#include <deque>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

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

using Portal = std::string;

void RegisterPortal(
    std::unordered_map<Portal, std::pair<Coord, Coord>> *portals,
    std::unordered_map<Coord, Portal> *portals_reverse, Portal const &p,
    Coord coord) {
  portals_reverse->emplace(coord, p);

  auto it = portals->find(p);
  if (it == portals->end()) {
    portals->emplace(p, std::make_pair(coord, Coord{0, 0}));
    return;
  }
  assert(it->second.second.x == 0 && it->second.second.y == 0);
  it->second.second = coord;
}

void ParsePortalAndAdjust(
    std::vector<char> &map,
    std::unordered_map<Portal, std::pair<Coord, Coord>> *portals,
    std::unordered_map<Coord, Portal> *portals_reverse, int width, int x,
    int y) {
  auto &c1 = map[y * width + x];
  assert(c1 >= 'A' && c1 <= 'Z');
  Portal p;
  p.push_back(c1);
  c1 = ' ';

  auto &c2a = map[y * width + x + 1];
  auto &c2b = map[(y + 1) * width + x];

  if (c2a >= 'A' && c2a <= 'Z') {
    p.push_back(c2a);
    c2a = ' ';

    if (map[y * width + x - 1] == '.') {
      map[y * width + x - 1] = 'p';
      RegisterPortal(portals, portals_reverse, p, {x - 1, y});
    } else {
      assert(map[y * width + x + 2] == '.');
      map[y * width + x + 2] = 'p';
      RegisterPortal(portals, portals_reverse, p, {x + 2, y});
    }
  } else {
    assert(c2b >= 'A' && c2b <= 'Z');
    p.push_back(c2b);
    c2b = ' ';

    if (map[(y - 1) * width + x] == '.') {
      map[(y - 1) * width + x] = 'p';
      RegisterPortal(portals, portals_reverse, p, {x, y - 1});
    } else {
      assert(map[(y + 2) * width + x] == '.');
      map[(y + 2) * width + x] = 'p';
      RegisterPortal(portals, portals_reverse, p, {x, y + 2});
    }
  }
}

int64_t
Search(std::vector<char> const &map,
       std::unordered_map<Portal, std::pair<Coord, Coord>> const &portals,
       std::unordered_map<Coord, Portal> const &portals_reverse, int width) {
  std::vector<int64_t> grid(map.size(), -1);

  std::deque<Coord> to_check;
  auto const start_pos = portals.at("AA").first;
  auto const end_pos = portals.at("ZZ").first;
  to_check.push_back(start_pos);
  grid[start_pos.y * width + start_pos.x] = 0;

  auto Step = [&map, &grid, &to_check, width](Coord c, int64_t steps) {
    auto const i = c.y * width + c.x;
    if (map[i] != '.' && map[i] != 'p') {
      return;
    }
    if (grid[i] != -1 && grid[i] <= steps + 1) {
      return;
    }
    grid[i] = steps + 1;
    to_check.push_back(c);
  };

  while (!to_check.empty()) {
    auto const pos = to_check.front();
    to_check.pop_front();

    auto const i = pos.y * width + pos.x;
    auto const s = grid[i];

    if (pos == end_pos) {
      return s;
    }

    if (map[i] == 'p') {
      auto const portal = portals_reverse.at(pos);
      auto const &portal_coords = portals.at(portal);
      auto const &c = ((portal_coords.first == pos) ? portal_coords.second
                                                    : portal_coords.first);
      Step(c, s);
    }

    Step({pos.x + 1, pos.y}, s);
    Step({pos.x - 1, pos.y}, s);
    Step({pos.x, pos.y + 1}, s);
    Step({pos.x, pos.y - 1}, s);
  }
  return -1;
}

int main() {
  std::vector<char> map;
  int width;
  int height;
  for (std::string line; std::getline(std::cin, line);) {
    width = line.size();
    std::copy(line.begin(), line.end(), std::back_inserter(map));
  }
  height = map.size() / width;

  std::unordered_map<Portal, std::pair<Coord, Coord>> portals;
  std::unordered_map<Coord, Portal> portals_reverse;

  for (int y = 0; y < height; ++y) {
    for (int x = 0; x < width; ++x) {
      auto const c = map[y * width + x];
      if (c >= 'A' && c <= 'Z') {
        ParsePortalAndAdjust(map, &portals, &portals_reverse, width, x, y);
      }
    }
  }

  auto const s = Search(map, portals, portals_reverse, width);
  std::cout << s << "\n";
  return 0;
}
