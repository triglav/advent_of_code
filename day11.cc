#include <cstdint>
#include <deque>
#include <iostream>
#include <optional>
#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>

using Memory = std::vector<int64_t>;

enum class Signal {
  END = 0,
  ERR,
  INT,
};

class Program {
public:
  explicit Program(Memory const &memory)
      : memory_(memory), pos_(0), relative_base_(0), state_(Signal::INT) {}

  std::tuple<Signal, int64_t> Execute(std::deque<int64_t> *inputs) {
    if (state_ != Signal::INT) {
      return std::make_tuple(state_, 0);
    }
    while (true) {
      auto const instruction = ReadMemory(pos_++);
      auto const opcode = instruction % 100;
      switch (opcode) {
      case 99:
        state_ = Signal::END;
        return std::make_tuple(Signal::END, 0);
      case 1: {
        auto const p1 = GetParamValue(1, instruction);
        auto const p2 = GetParamValue(2, instruction);
        auto const output_pos = GetParamAddress(3, instruction);
        WriteMemory(output_pos, p1 + p2);
        break;
      }
      case 2: {
        auto const p1 = GetParamValue(1, instruction);
        auto const p2 = GetParamValue(2, instruction);
        auto const output_pos = GetParamAddress(3, instruction);
        WriteMemory(output_pos, p1 * p2);
        break;
      }
      case 3: {
        auto const output_pos = GetParamAddress(1, instruction);
        WriteMemory(output_pos, inputs->front());
        inputs->pop_front();
        break;
      }
      case 4: {
        auto const x = GetParamValue(1, instruction);
        state_ = Signal::INT;
        return std::make_tuple(Signal::INT, x);
      }
      case 5: {
        auto const p1 = GetParamValue(1, instruction);
        auto const p2 = GetParamValue(2, instruction);
        if (p1 != 0) {
          pos_ = p2;
        }
        break;
      }
      case 6: {
        auto const p1 = GetParamValue(1, instruction);
        auto const p2 = GetParamValue(2, instruction);
        if (p1 == 0) {
          pos_ = p2;
        }
        break;
      }
      case 7: {
        auto const p1 = GetParamValue(1, instruction);
        auto const p2 = GetParamValue(2, instruction);
        auto const output_pos = GetParamAddress(3, instruction);
        WriteMemory(output_pos, ((p1 < p2) ? 1 : 0));
        break;
      }
      case 8: {
        auto const p1 = GetParamValue(1, instruction);
        auto const p2 = GetParamValue(2, instruction);
        auto const output_pos = GetParamAddress(3, instruction);
        WriteMemory(output_pos, ((p1 == p2) ? 1 : 0));
        break;
      }
      case 9: {
        auto const p = GetParamValue(1, instruction);
        relative_base_ += p;
        break;
      }
      default:
        std::cerr << "Invalid opcode: " << opcode << "\n";
        state_ = Signal::ERR;
        return std::make_tuple(Signal::ERR, 0);
      }
    }
    std::cerr << "Unexpected error\n";
    state_ = Signal::ERR;
    return std::make_tuple(Signal::ERR, 0);
  }

private:
  int64_t GetParamAddress(int64_t number, int64_t instruction) {
    auto const mode =
        (instruction / static_cast<int64_t>(std::pow(10, number + 1))) % 10;
    if (mode == 0) {
      auto const input_pos = ReadMemory(pos_++);
      return input_pos;
    }
    if (mode == 2) {
      auto const offset = ReadMemory(pos_++);
      return relative_base_ + offset;
    }
    assert(mode == 1);
    return pos_++;
  }

  int64_t GetParamValue(int64_t number, int64_t instruction) {
    auto const address = GetParamAddress(number, instruction);
    return ReadMemory(address);
  }

  void WriteMemory(int64_t p, int64_t value) {
    if (memory_.size() <= p) {
      memory_.resize(p + 1, 0);
    }
    memory_[p] = value;
  }

  int64_t ReadMemory(int64_t p) {
    if (memory_.size() <= p) {
      memory_.resize(p + 1, 0);
    }
    return memory_[p];
  }

private:
  Memory memory_;
  int64_t pos_;
  int64_t relative_base_;
  Signal state_;
}; // class Program

enum class Color {
  Black = 0,
  White = 1,
};

enum class TurnDirection {
  Left = 0,
  Right = 1,
};

enum class Direction {
  Up = 0,
  Right = 1,
  Down = 2,
  Left = 3,
};

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

class Robot {
public:
  explicit Robot(Memory const &memory)
      : program_(memory), direction_(Direction::Up) {}

  std::optional<Color> Advance(Color panel_color) {
    std::deque<int64_t> inputs = {static_cast<int64_t>(panel_color)};
    auto const r1 = program_.Execute(&inputs);
    auto const s1 = std::get<0>(r1);
    if (s1 == Signal::END) {
      return std::nullopt;
    }
    if (s1 == Signal::ERR) {
      std::cerr << "oops 1\n";
      return std::nullopt;
    }
    auto const r2 = program_.Execute(&inputs);
    auto const s2 = std::get<0>(r2);
    if (s2 == Signal::END) {
      return std::nullopt;
    }
    if (s2 == Signal::ERR) {
      std::cerr << "oops 2\n";
      return std::nullopt;
    }
    auto const turn = static_cast<TurnDirection>(std::get<1>(r2));
    Turn(turn);
    Step();
    auto const color = static_cast<Color>(std::get<1>(r1));
    return color;
  }

  Coord pos() const { return pos_; }

private:
  void Turn(TurnDirection turn_direction) {
    if (turn_direction == TurnDirection::Left) {
      direction_ =
          static_cast<Direction>((static_cast<int>(direction_) - 1 + 4) % 4);
    } else {
      direction_ =
          static_cast<Direction>((static_cast<int>(direction_) + 1) % 4);
    }
  }
  void Step() {
    switch (direction_) {
    case Direction::Up:
      pos_ = {pos_.x, pos_.y - 1};
      break;
    case Direction::Right:
      pos_ = {pos_.x - 1, pos_.y};
      break;
    case Direction::Down:
      pos_ = {pos_.x, pos_.y + 1};
      break;
    case Direction::Left:
      pos_ = {pos_.x + 1, pos_.y};
      break;
    }
  }

private:
  Program program_;
  Direction direction_;
  Coord pos_;
}; // class Robot

int main() {
  Memory memory;
  {
    std::string token;
    while (std::getline(std::cin, token, ',')) {
      auto instruction = std::stol(token);
      memory.push_back(instruction);
    }
  }

  Robot robot{memory};

  std::unordered_map<Coord, Color> grid;
  auto GetColor = [&grid](Coord pos) -> Color {
    auto it = grid.find(pos);
    if (it == grid.end()) {
      return Color::Black;
    }
    return it->second;
  };

  while (true) {
    auto const pos = robot.pos();
    auto const color = GetColor(pos);
    auto const to_paint = robot.Advance(color);
    if (!to_paint) {
      break;
    }
    grid[pos] = *to_paint;
  }
  std::cout << grid.size() << "\n";
  return 0;
}

