#include <algorithm>
#include <numeric>
#include <array>
#include <iostream>

void SimulateDay(std::array<uint64_t, 9> *lanternfish) {
  std::rotate(lanternfish->begin(), lanternfish->begin() + 1,
              lanternfish->end());
  (*lanternfish)[6] += (*lanternfish)[8];
}

int main() {
  std::array<uint64_t, 9> lanternfish = {0, 0, 0, 0, 0, 0, 0, 0, 0};

  std::string token;
  while (std::getline(std::cin, token, ',')) {
    auto const f = std::stoi(token);
    lanternfish[f] += 1;
  }

  for (int i = 0; i < 256; ++i) {
    SimulateDay(&lanternfish);
  }
  std::cout << std::accumulate(lanternfish.begin(), lanternfish.end(), 0UL) << "\n";
  return 0;
}
