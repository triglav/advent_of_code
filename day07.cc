#include <cmath>
#include <deque>
#include <iostream>
#include <string>
#include <vector>

int GetParamValue(int pos, int number, int instruction,
                  std::vector<int> const &memory) {
  auto const mode =
      (instruction / static_cast<int>(std::pow(10, number + 1))) % 10;
  if (mode == 0) {
    auto const input_pos = memory[pos + number];
    return memory[input_pos];
  }
  assert(mode == 1);
  return memory[pos + number];
}

int ExecuteProgram(std::deque<int> inputs, std::vector<int> memory) {
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
      memory[output_pos] = inputs.front();
      inputs.pop_front();
      pos += 2;
    } else if (opcode == 4) {
      auto const output_pos = memory[pos + 1];
      pos += 2;
      return memory[output_pos];
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
  exit(1);
}

using PhaseSetting = std::vector<int>;

void GeneratePhaseSettingVariations_step(
    std::vector<PhaseSetting> *setting_variations,
    std::vector<int> const &available_phases, PhaseSetting const &setting) {
  if (available_phases.empty()) {
    setting_variations->push_back(setting);
    return;
  }

  for (auto phase : available_phases) {
    auto phases = available_phases;
    phases.erase(std::remove(phases.begin(), phases.end(), phase),
                 phases.end());
    auto setting2 = setting;
    setting2.push_back(phase);
    GeneratePhaseSettingVariations_step(setting_variations, phases, setting2);
  }
}

std::vector<PhaseSetting>
GeneratePhaseSettingVariations(std::vector<int> const &available_phases) {
  std::vector<PhaseSetting> setting_variations;
  GeneratePhaseSettingVariations_step(&setting_variations, available_phases,
                                      {});
  return setting_variations;
}

int ExecuteAplifiers(std::vector<int> const &program,
                     PhaseSetting const &setting, int input_signal) {
  auto signal = input_signal;
  for (auto phase : setting) {
    signal = ExecuteProgram({phase, signal}, program);
  }
  return signal;
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

  auto const setting_variations =
      GeneratePhaseSettingVariations({0, 1, 2, 3, 4});
  int max_result = 0;
  for (auto const &setting : setting_variations) {
    max_result = std::max(ExecuteAplifiers(program, setting, 0), max_result);
  }
  std::cout << max_result << "\n";

  return 0;
}
