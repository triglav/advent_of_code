#include <cassert>
#include <iostream>
#include <vector>

#include "string_utils.h"

int main() {
  int64_t register_a = 0;
  int64_t register_b = 0;
  std::vector<std::string> program;

  std::string line;
  while (std::getline(std::cin, line)) {
    program.push_back(line);
  }

  size_t pos = 0;
  while (pos < program.size()) {
    assert(pos >= 0);
    auto const &instruction = program[pos];

    if (instruction.rfind("hlf", 0) == 0) {
      auto &r = ((instruction[4] == 'a') ? register_a : register_b);
      r /= 2;
    } else if (instruction.rfind("tpl", 0) == 0) {
      auto &r = ((instruction[4] == 'a') ? register_a : register_b);
      r *= 3;
    } else if (instruction.rfind("inc", 0) == 0) {
      auto &r = ((instruction[4] == 'a') ? register_a : register_b);
      ++r;
    } else if (instruction.rfind("jmp", 0) == 0) {
      auto const offset = sv2number<int>(Trim(instruction.substr(4), "+"));
      pos += offset;
      continue;
    } else if (instruction.rfind("jie", 0) == 0) {
      auto const &r = ((instruction[4] == 'a') ? register_a : register_b);
      if (r % 2 == 0) {
        auto const offset = sv2number<int>(Trim(instruction.substr(7), "+"));
        pos += offset;
        continue;
      }
    } else if (instruction.rfind("jio", 0) == 0) {
      auto const &r = ((instruction[4] == 'a') ? register_a : register_b);
      if (r == 1) {
        auto const offset = sv2number<int>(Trim(instruction.substr(7), "+"));
        pos += offset;
        continue;
      }
    }
    ++pos;
  }
  std::cout << register_b << "\n";
  return 0;
}
