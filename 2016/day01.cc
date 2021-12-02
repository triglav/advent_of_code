#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <tuple>

enum class Direction {
  N = 0,
  W,
  S,
  E,
  Count,
};

constexpr Direction TurnRight(Direction d) {
  return static_cast<Direction>((static_cast<int>(d) + 1) % static_cast<int>(Direction::Count));
}
constexpr Direction TurnLeft(Direction d) {
  return static_cast<Direction>((static_cast<int>(d) - 1 + static_cast<int>(Direction::Count)) %
      static_cast<int>(Direction::Count));
}

constexpr std::tuple<int, int> WalkForward(std::tuple<int, int> pos, Direction d, int blocks) {
  auto const [x, y] = pos;
  switch (d) {
  case Direction::N:
    return std::make_tuple(x, y-blocks);
  case Direction::W:
    return std::make_tuple(x+blocks, y);
  case Direction::S:
    return std::make_tuple(x, y+blocks);
  case Direction::E:
    return std::make_tuple(x-blocks, y);
  default:
    throw std::logic_error("invalid direction");
  }
}


int main() {
  auto direction = Direction::N;
  auto position = std::make_tuple(0, 0);

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
    position = WalkForward(position, direction, std::stoi(blocks));
  }
  auto const [x, y] = position;
  std::cout << std::abs(x) + std::abs(y) << "\n";
  return 0;
}
