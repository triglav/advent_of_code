#include <cmath>
#include <deque>
#include <iostream>
#include <string>
#include <tuple>
#include <vector>

using Memory = std::vector<int>;

enum class Signal {
  END = 0,
  ERR,
  INT,
};

class Program {
public:
  explicit Program(Memory const &memory)
      : memory_(memory), pos_(0), state_(Signal::INT) {}

  std::tuple<Signal, int> Execute(std::deque<int> *inputs) {
    if (state_ != Signal::INT) {
      return std::make_tuple(state_, 0);
    }
    while (true) {
      auto const instruction = memory_[pos_++];
      auto const opcode = instruction % 100;
      if (opcode == 99) {
        state_ = Signal::END;
        return std::make_tuple(Signal::END, 0);
      }
      if (opcode == 1) {
        auto const p1 = GetParamValue(1, instruction);
        auto const p2 = GetParamValue(2, instruction);
        auto const output_pos = memory_[pos_++];
        memory_[output_pos] = p1 + p2;
      } else if (opcode == 2) {
        auto const p1 = GetParamValue(1, instruction);
        auto const p2 = GetParamValue(2, instruction);
        auto const output_pos = memory_[pos_++];
        memory_[output_pos] = p1 * p2;
      } else if (opcode == 3) {
        auto const output_pos = memory_[pos_++];
        memory_[output_pos] = inputs->front();
        inputs->pop_front();
      } else if (opcode == 4) {
        auto const output_pos = memory_[pos_++];
        state_ = Signal::INT;
        return std::make_tuple(Signal::INT, memory_[output_pos]);
      } else if (opcode == 5) {
        auto const p1 = GetParamValue(1, instruction);
        auto const p2 = GetParamValue(2, instruction);
        if (p1 != 0) {
          pos_ = p2;
        }
      } else if (opcode == 6) {
        auto const p1 = GetParamValue(1, instruction);
        auto const p2 = GetParamValue(2, instruction);
        if (p1 == 0) {
          pos_ = p2;
        }
      } else if (opcode == 7) {
        auto const p1 = GetParamValue(1, instruction);
        auto const p2 = GetParamValue(2, instruction);
        auto const output_pos = memory_[pos_++];
        memory_[output_pos] = ((p1 < p2) ? 1 : 0);
      } else if (opcode == 8) {
        auto const p1 = GetParamValue(1, instruction);
        auto const p2 = GetParamValue(2, instruction);
        auto const output_pos = memory_[pos_++];
        memory_[output_pos] = ((p1 == p2) ? 1 : 0);
      } else {
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
  int GetParamValue(int number, int instruction) {
    auto const mode =
        (instruction / static_cast<int>(std::pow(10, number + 1))) % 10;
    if (mode == 0) {
      auto const input_pos = memory_[pos_++];
      return memory_[input_pos];
    }
    assert(mode == 1);
    return memory_[pos_++];
  }

private:
  Memory memory_;
  int pos_;
  Signal state_;
}; // class Program

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

int ExecuteAplifiers(Memory const &memory, PhaseSetting const &setting,
                     int input_signal) {
  auto signal = input_signal;
  for (auto phase : setting) {
    Program program{memory};
    std::deque<int> inputs = {phase, signal};
    auto const result = program.Execute(&inputs);
    if (std::get<0>(result) != Signal::INT) {
      exit(1);
    }
    signal = std::get<1>(result);
  }
  return signal;
}

int FeedbackLoopAplifiers(Memory const &memory, PhaseSetting const &setting,
                          int input_signal) {
  std::vector<Program> amplifiers(setting.size(), Program(memory));

  std::vector<std::deque<int>> inputs(setting.size());
  for (int i = 0; i < setting.size(); ++i) {
    inputs[i].push_back(setting[i]);
  }
  inputs.front().push_back(input_signal);

  int i = 0;
  while (true) {
    auto const r = amplifiers[i].Execute(&inputs[i]);
    i = (i + 1) % setting.size();
    auto s = std::get<0>(r);
    if (s == Signal::ERR) {
      exit(1);
    }
    if (s == Signal::INT) {
      auto const x = std::get<1>(r);
      inputs[i].push_back(x);
      continue;
    }
    if (s == Signal::END) {
      if (i + 1 == setting.size()) {
        break;
      }
      continue;
    }
  }
  return inputs.front().front();
}

int main() {
  Memory memory;
  {
    std::string token;
    while (std::getline(std::cin, token, ',')) {
      auto instruction = std::stoi(token);
      memory.push_back(instruction);
    }
  }

  {
    auto const setting_variations =
        GeneratePhaseSettingVariations({0, 1, 2, 3, 4});
    int max_result = 0;
    for (auto const &setting : setting_variations) {
      max_result = std::max(ExecuteAplifiers(memory, setting, 0), max_result);
    }
    std::cout << max_result << "\n";
  }
  {
    auto const setting_variations =
        GeneratePhaseSettingVariations({5, 6, 7, 8, 9});
    int max_result = 0;
    for (auto const &setting : setting_variations) {
      max_result =
          std::max(FeedbackLoopAplifiers(memory, setting, 0), max_result);
    }
    std::cout << max_result << "\n";
  }

  return 0;
}
