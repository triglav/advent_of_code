#include <iostream>
#include <string>
#include <vector>

int ExecuteProgram(int noun, int verb, std::vector<int> memory) {
  memory[1] = noun;
  memory[2] = verb;

  int pos = 0;
  while (true) {
    auto opcode = memory[pos];
    if (opcode == 99) {
      break;
    }
    auto input_pos1 = memory[pos + 1];
    auto input_pos2 = memory[pos + 2];
    auto output_pos = memory[pos + 3];

    if (opcode == 1) {
      memory[output_pos] = memory[input_pos1] + memory[input_pos2];
    } else if (opcode == 2) {
      memory[output_pos] = memory[input_pos1] * memory[input_pos2];
    } else {
      std::cerr << "Invalid opcode: " << opcode << "\n";
      break;
    }
    pos += 4;
  }
  return memory[0];
}

int main() {
  std::vector<int> program;
  {
    std::string token;
    while (std::getline(std::cin, token, ',')) {
      auto opcode = std::stoi(token);
      program.push_back(opcode);
    }
  }

  auto result = ExecuteProgram(12, 2, program);
  std::cout << result << "\n";
  return 0;
}

