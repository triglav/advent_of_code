#include <cmath>
#include <iostream>
#include <string>
#include <vector>

int GetParamValue(int pos, int number, int instruction, std::vector<int> const & memory) {
  auto const mode = (instruction / static_cast<int>(std::pow(10, number + 1))) % 10;
  if (mode == 0) {
    auto const input_pos = memory[pos + number];
    return memory[input_pos];
  }
  assert(mode == 1);
  return memory[pos + number];
}

void ExecuteProgram(int input_param, std::vector<int> memory) {
  int pos = 0;
  while (true) {
    auto const instruction = memory[pos];
    auto const opcode = instruction % 100;
    if (opcode == 99) {
      break;
    }
    if (opcode == 1) {
      auto const p1 = GetParamValue(pos, 1, instruction, memory);
      auto const p2 = GetParamValue(pos, 2, instruction, memory);
      auto const output_pos = memory[pos + 3];
      memory[output_pos] = p1 + p2;
      pos += 4;
    } else if (opcode == 2) {
      auto const p1 = GetParamValue(pos, 1, instruction, memory);
      auto const p2 = GetParamValue(pos, 2, instruction, memory);
      auto const output_pos = memory[pos + 3];
      memory[output_pos] = p1 * p2;
      pos += 4;
    } else if (opcode == 3) {
      auto const output_pos = memory[pos + 1];
      memory[output_pos] = input_param;
      pos += 2;
    } else if (opcode == 4) {
      auto const output_pos = memory[pos + 1];
      std::cout << memory[output_pos] << "\n";
      pos += 2;
    } else if (opcode == 5) {
      auto const p1 = GetParamValue(pos, 1, instruction, memory);
      auto const p2 = GetParamValue(pos, 2, instruction, memory);
      if (p1 != 0) {
        pos = p2;
      } else {
        pos += 3;
      }
    } else if (opcode == 6) {
      auto const p1 = GetParamValue(pos, 1, instruction, memory);
      auto const p2 = GetParamValue(pos, 2, instruction, memory);
      if (p1 == 0) {
        pos = p2;
      } else {
        pos += 3;
      }
    } else if (opcode == 7) {
      auto const p1 = GetParamValue(pos, 1, instruction, memory);
      auto const p2 = GetParamValue(pos, 2, instruction, memory);
      auto const output_pos = memory[pos + 3];
      memory[output_pos] = ((p1 < p2) ? 1 : 0);
      pos += 4;
    } else if (opcode == 8) {
      auto const p1 = GetParamValue(pos, 1, instruction, memory);
      auto const p2 = GetParamValue(pos, 2, instruction, memory);
      auto const output_pos = memory[pos + 3];
      memory[output_pos] = ((p1 == p2) ? 1 : 0);
      pos += 4;
    } else {
      std::cerr << "Invalid opcode: " << opcode << "\n";
      break;
    }
  }
}

int main() {
  std::vector<int> program;
  {
    std::string token;
    while (std::getline(std::cin, token, ',')) {
      auto instruction = std::stoi(token);
      program.push_back(instruction);
    }
  }
  ExecuteProgram(1, program);
  std::cout << "==\n";
  ExecuteProgram(5, program);
  return 0;
}

