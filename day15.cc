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

  std::tuple<Signal, int64_t> Execute(int input) {
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
        WriteMemory(output_pos, input);
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

enum OutputCode {
  kWallHit = 0,
  kStep = 1,
  kStepAndOxygenSystem = 2,
};

enum class InputCode : int {
  kNorth = 1,
  kSouth = 2,
  kWest = 3,
  kEast = 4,
};

std::vector<InputCode> inputs = {InputCode::kNorth, InputCode::kSouth,
                                 InputCode::kWest, InputCode::kEast};

using Grid = std::unordered_map<Coord, int64_t>;

std::optional<int64_t> GetTile(Grid const &grid, Coord const &pos) {
  auto it = grid.find(pos);
  if (it == grid.end()) {
    return std::nullopt;
  }
  return it->second;
};

enum class ResultCode {
  CarryOn,
  NoPoint,
  Found,
};

class Robot {
public:
  Robot(Memory const &memory) : program_(memory), pos_{0, 0}, steps_(0) {}
  ~Robot() = default;

  Robot(Robot const &) = default;
  Robot &operator=(Robot const &) = default;

  ResultCode Walk(Grid *grid, InputCode input) {
    auto const [signal, value] = program_.Execute(static_cast<int>(input));
    assert(signal == Signal::INT);
    auto const o = static_cast<OutputCode>(value);
    switch (o) {
    case OutputCode::kStep: {
      ++steps_;
      pos_ = InputCode2Coord(input);
      auto tile = GetTile(*grid, pos_);
      if (!tile.has_value() || *tile > steps_) {
        (*grid)[pos_] = steps_;
        return ResultCode::CarryOn;
      }
      return ResultCode::NoPoint;
    } break;
    case OutputCode::kStepAndOxygenSystem:
      ++steps_;
      pos_ = InputCode2Coord(input);
      return ResultCode::Found;
    case OutputCode::kWallHit:
    default:
      return ResultCode::NoPoint;
    }
    return ResultCode::NoPoint;
  }

  Coord const &pos() const { return pos_; }
  int64_t steps() const { return steps_; }

private:
  constexpr Coord InputCode2Coord(InputCode input) const {
    switch (input) {
    case InputCode::kNorth:
      return {pos_.x, pos_.y - 1};
    case InputCode::kSouth:
      return {pos_.x, pos_.y + 1};
    case InputCode::kWest:
      return {pos_.x - 1, pos_.y};
    case InputCode::kEast:
      return {pos_.x + 1, pos_.y};
    }
  }

private:
  Program program_;
  Coord pos_;
  int64_t steps_;
}; // Robot

int main() {
  Memory memory;
  {
    std::string token;
    while (std::getline(std::cin, token, ',')) {
      auto instruction = std::stol(token);
      memory.push_back(instruction);
    }
  }
  Grid grid;
  grid[{0, 0}] = 0;

  std::deque<Robot> to_check;
  to_check.push_back(Robot{memory});
  while (!to_check.empty()) {
    auto r = to_check.front();
    to_check.pop_front();

    bool f = false;
    for (auto i : inputs) {
      auto r1 = r;
      auto const x = r1.Walk(&grid, i);
      if (x == ResultCode::CarryOn) {
        to_check.push_back(r1);
        continue;
      }
      if (x == ResultCode::Found) {
        std::cout << r1.steps() << "\n";
        f = true;
        break;
      }
    }
    if (f) {
      break;
    }
  }
  return 0;
}
