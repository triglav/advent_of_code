#include <iostream>
#include <vector>

void SimulateDay(std::vector<int> *lanternfish) {
  int new_fish = 0;
  for (auto it = lanternfish->begin(); it != lanternfish->end(); ++it) {
    auto &f = *it;
    --f;
    if (f < 0) {
      f = 6;
      ++new_fish;
    }
  }
  for (int i = 0; i < new_fish; ++i) {
    lanternfish->push_back(8);
  }
}

int main() {
  std::vector<int> lanternfish;

  std::string token;
  while (std::getline(std::cin, token, ',')) {
    auto const f = std::stoi(token);
    lanternfish.push_back(f);
  }

  for (int i = 0; i < 80; ++i) {
    SimulateDay(&lanternfish);
  }
  std::cout << lanternfish.size() << "\n";
  return 0;
}
