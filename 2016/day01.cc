#include <iostream>
#include <limits>
#include <sstream>
#include <stdexcept>
#include <string>
#include <unordered_map>

#include "coord.h"

enum class Direction {
  N = 0,
  W,
  S,
  E,
  Count,
};

constexpr Direction TurnRight(Direction d) {
  return static_cast<Direction>((static_cast<int>(d) + 1) %
                                static_cast<int>(Direction::Count));
}
constexpr Direction TurnLeft(Direction d) {
  return static_cast<Direction>(
      (static_cast<int>(d) - 1 + static_cast<int>(Direction::Count)) %
      static_cast<int>(Direction::Count));
}

constexpr Coord WalkForward(Coord pos, Direction d, int blocks) {
  auto const [x, y] = pos;
  switch (d) {
  case Direction::N:
    return Coord{x, y - blocks};
  case Direction::W:
    return Coord{x + blocks, y};
  case Direction::S:
    return Coord{x, y + blocks};
  case Direction::E:
    return Coord{x - blocks, y};
  default:
    throw std::logic_error("invalid direction");
  }
}

Coord WalkForwardAndMarkVisited(std::unordered_map<Coord, int> *visited,
                                Coord pos, Direction d, int blocks) {
  auto p = pos;
  for (int i = 0; i < blocks; ++i) {
    p = WalkForward(p, d, 1);
    auto const it = visited->find(p);
    auto const c = (it != visited->end() ? it->second : 0);
    visited->insert_or_assign(p, c + 1);
  }
  return p;
}

int main() {
  auto direction = Direction::N;
  auto position = Coord{0, 0};

  std::unordered_map<Coord, int> visited;
  visited.insert({position, 1});

  std::string token;
  while (std::cin >> token) {
    std::stringstream ss(token);

    char turn;
    ss >> turn;
    if (turn == 'L') {
      direction = TurnLeft(direction);
    } else if (turn == 'R') {
      direction = TurnRight(direction);
    } else {
      throw std::logic_error("invalid direction input");
    }

    std::string blocks;
    std::getline(ss, blocks, ',');
    position = WalkForwardAndMarkVisited(&visited, position, direction,
                                         std::stoi(blocks));
  }
  auto const [x, y] = position;
  std::cout << std::abs(x) + std::abs(y) << "\n";

  int distance = std::numeric_limits<int>::max();
  for (auto i = visited.begin(); i != visited.end(); ++i) {
    if (i->second < 2) {
      continue;
    }
    auto const p = i->first;
    auto const d = std::abs(p.x) + std::abs(p.y);
    if (d < distance) {
      distance = d;
    }
  }
  std::cout << distance << "\n";
  return 0;
}
