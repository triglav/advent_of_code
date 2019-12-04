#include <functional>
#include <iostream>
#include <limits>
#include <set>
#include <sstream>
#include <string>

void ReadWireInput(std::function<void(int, int)> handle_path) {
  std::string line;
  std::cin >> line;
  std::stringstream line_stream(line);
  std::string token;
  while (std::getline(line_stream, token, ',')) {
    auto code = token[0];
    auto distance = std::stoi(token.substr(1));

    handle_path(code, distance);
  }
}

struct Point {
  int x;
  int y;
};

struct PointCompare {
  constexpr bool operator()(Point const & lhs, Point const & rhs) const {
    if (lhs.x < rhs.x) {
      return true;
    }
    if (lhs.x > rhs.x) {
      return false;
    }
    return lhs.y < rhs.y;
  }
};

Point Step(Point pos, char code) {
  switch (code) {
  case 'R':
    ++pos.x;
    break;
  case 'U':
    ++pos.y;
    break;
  case 'L':
    --pos.x;
    break;
  case 'D':
    --pos.y;
    break;
  default:
    exit(1);
    break;
  }
  return pos;
}

int main() {
  std::set<Point, PointCompare> grid;

  Point pos{0, 0};
  ReadWireInput([&grid, &pos] (int code, int distance) {
    for (int i = 0; i < distance; ++i) {
      pos = Step(pos, code);
      grid.insert(pos);
    }
  });

  auto smallest_distance = std::numeric_limits<int>::max();
  pos = {0, 0};
  ReadWireInput([&grid, &pos, &smallest_distance] (int code, int distance) {
    for (int i = 0; i < distance; ++i) {
      pos = Step(pos, code);
      if (grid.find(pos) != grid.end()) {
        smallest_distance = std::min(smallest_distance, std::abs(pos.x) + std::abs(pos.y));
      }
    }
  });
  std::cout << smallest_distance << "\n";
  return 0;
}

