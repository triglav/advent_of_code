#include <cassert>
#include <cmath>
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

  std::tuple<Signal, int64_t> Execute(std::deque<char> *inputs) {
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

bool IsIntersection(std::vector<char> const &grid, int width, int x, int y) {
  auto const idx = y * width + x;
  if (grid[idx] != '#') {
    return false;
  }
  return grid[idx + 1] == '#' && grid[idx - 1] == '#' &&
         grid[idx + width] == '#' && grid[idx - width] == '#';
}

int main() {
  Memory memory;
  {
    std::string token;
    while (std::getline(std::cin, token, ',')) {
      auto instruction = std::stol(token);
      memory.push_back(instruction);
    }
  }
  {
    Program program{memory};

    std::vector<char> grid;
    int width = 0;
    int height = 0;
    while (true) {
      auto const [s, v] = program.Execute(0);
      if (s == Signal::END) {
        break;
      }
      if (s == Signal::ERR) {
        std::cerr << "oops\n";
        break;
      }
      auto const c = static_cast<char>(v);
      if (c != '\n') {
        grid.push_back(c);
      } else if (width == 0) {
        width = grid.size();
      }
    }
    height = grid.size() / width;

    int64_t s = 0;
    for (int y = 1; y < height - 1; ++y) {
      for (int x = 1; x < width - 1; ++x) {
        if (IsIntersection(grid, width, x, y)) {
          s += x * y;
        }
      }
    }
    std::cout << s << "\n";
  }
  {
    auto memory2 = memory;
    memory2[0] = 2;
    Program program{memory2};

    std::deque<char> inputs;
    for (auto c : "A,B,B,A,C,B,C,C,B,A\n"
                  "R,10,R,8,L,10,L,10\n"
                  "R,8,L,6,L,6\n"
                  "L,10,R,10,L,6\n"
                  "n\n") {
      inputs.push_back(c);
    }
    int64_t r;
    while (true) {
      auto const [s, v] = program.Execute(&inputs);
      if (s == Signal::END) {
        break;
      }
      if (s == Signal::ERR) {
        std::cerr << "oops\n";
        break;
      }
      if (v > 256) {
        r = v;
      }
    }
    std::cout << r << "\n";
  }
  return 0;
}
