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

int FindParams(int desired_result, std::vector<int> const & memory) {
  for (int noun = 0; noun < 100; ++noun) {
    for (int verb = 0; verb < 100; ++verb) {
      if (desired_result == ExecuteProgram(noun, verb, memory)) {
        return 100 * noun + verb;
      }
    }
  }
  return -1;
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

  auto noun_and_verb = FindParams(19690720, program);
  std::cout << noun_and_verb << "\n";

  return 0;
}

