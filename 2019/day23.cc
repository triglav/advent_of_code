#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdint>
#include <deque>
#include <iostream>
#include <optional>
#include <string>
#include <tuple>
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
        if (inputs->empty()) {
          state_ = Signal::INT;
          return std::make_tuple(Signal::INT, -1);
        }
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
    std::vector<Program> nic(50, Program{memory});
    std::vector<std::deque<int64_t>> inputs(nic.size());
    for (int i = 0; i < nic.size(); ++i) {
      inputs[i].push_back(i);
      inputs[i].push_back(-1);
    }
    bool done = false;
    while (!done) {
      for (int i = 0; i < nic.size(); ++i) {
        auto const [s, v] = nic[i].Execute(&inputs[i]);
        if (s == Signal::END) {
          std::cout << "end\n";
          exit(0);
        }
        if (s == Signal::ERR) {
          std::cerr << "oops\n";
          exit(1);
        }
        auto const [sx, x] = nic[i].Execute(&inputs[i]);
        auto const [sy, y] = nic[i].Execute(&inputs[i]);
        if (v == -1) {
          continue;
        }
        assert((v >= 0 && v < nic.size()) || v == 255);

        if (v == 255) {
          std::cout << y << "\n";
          done = true;
          break;
        } else {
          inputs[v].push_back(x);
          inputs[v].push_back(y);
        }
      }
    }
  }
  {
    std::vector<Program> nic(50, Program{memory});
    std::vector<std::deque<int64_t>> inputs(nic.size());

    std::optional<int64_t> nat_x_previous;
    std::optional<int64_t> nat_y_previous;

    int64_t nat_x;
    int64_t nat_y;
    for (int i = 0; i < nic.size(); ++i) {
      inputs[i].push_back(i);
      inputs[i].push_back(-1);
    }
    while (true) {
      for (int i = 0; i < nic.size(); ++i) {
        auto const [s, v] = nic[i].Execute(&inputs[i]);
        if (s == Signal::END) {
          std::cout << "end\n";
          exit(0);
        }
        if (s == Signal::ERR) {
          std::cerr << "oops\n";
          exit(1);
        }
        auto const [sx, x] = nic[i].Execute(&inputs[i]);
        auto const [sy, y] = nic[i].Execute(&inputs[i]);
        if (v == -1) {
          continue;
        }
        assert((v >= 0 && v < nic.size()) || v == 255);

        if (v == 255) {
          nat_x = x;
          nat_y = y;
        } else {
          inputs[v].push_back(x);
          inputs[v].push_back(y);
        }
      }
      if (std::all_of(inputs.begin(), inputs.end(),
                      [](std::deque<int64_t> const &i) { return i.empty(); })) {
        inputs[0].push_back(nat_x);
        inputs[0].push_back(nat_y);

        if (nat_x_previous.has_value() && nat_y_previous.has_value() &&
            *nat_x_previous == nat_x && *nat_y_previous == nat_y) {
          std::cout << nat_y << "\n";
          exit(0);
        }
        nat_x_previous = nat_x;
        nat_y_previous = nat_y;
      }
    }
  }
  return 0;
}
