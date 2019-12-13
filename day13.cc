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

  std::tuple<Signal, int64_t> Execute(int *input) {
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
        WriteMemory(output_pos, *input);
        *input = 0;
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

enum TileId {
  kEmpty = 0,
  kWall = 1,
  kBlock = 2,
  kPaddle = 3,
  kBall = 4,
};

int main() {
  Memory memory_orig;
  {
    std::string token;
    while (std::getline(std::cin, token, ',')) {
      auto instruction = std::stol(token);
      memory_orig.push_back(instruction);
    }
  }

  auto const kWidth = 37;
  auto const kHeight = 26;
  int paddle_pos = 0;
  int ball_pos = 0;
  {
    int block_count = 0;
    Memory memory{memory_orig};
    Program game{memory};
    while (true) {
      auto r1 = game.Execute({});
      auto const s1 = std::get<0>(r1);
      if (s1 == Signal::END) {
        break;
      }
      auto r2 = game.Execute({});
      auto r3 = game.Execute({});

      auto const x = std::get<1>(r1);
      auto const y = std::get<1>(r2);
      auto const tile_id = std::get<1>(r3);

      if (tile_id == kBlock) {
        ++block_count;
      } else if (tile_id == kBall) {
        ball_pos = x;
      } else if (tile_id == kPaddle) {
        paddle_pos = x;
      }
    }
    std::cout << block_count << "\n";
  }
  {
    Memory memory{memory_orig};
    memory[0] = 2;
    Program game{memory};

    int score = 0;
    int joystick = 0;

    while (true) {
      auto r1 = game.Execute(&joystick);
      auto const s1 = std::get<0>(r1);
      if (s1 == Signal::END) {
        break;
      }
      auto r2 = game.Execute(&joystick);
      auto r3 = game.Execute(&joystick);

      auto const x = std::get<1>(r1);
      auto const y = std::get<1>(r2);
      auto const tile_id = std::get<1>(r3);
      if (x == -1 && y == 0) {
        score = tile_id;
        continue;
      }
      if (tile_id == kBall) {
        ball_pos = x;
        if (paddle_pos > ball_pos) {
          joystick = -1;
        } else if (paddle_pos < ball_pos) {
          joystick = 1;
        }
      }
      if (tile_id == kPaddle) {
        paddle_pos = x;
      }
    }
    std::cout << score << "\n";
  }
  return 0;
}

