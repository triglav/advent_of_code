#include <cmath>
#include <iostream>
#include <vector>

int64_t CalculatePresents(int64_t house) {
  int64_t count = 0;
  for (int64_t i = 1; i <= std::sqrt(house); ++i) {
    if (house % i == 0) {
      count += i * 10;
      if (house / i != i) {
        count += (house / i) * 10;
      }
    }
  }
  return count;
}

int main() {
  int64_t const kRequiredPresents = 34000000;

  int64_t house = 1;
  while (true) {
    auto const presents = CalculatePresents(house);
    if (presents >= kRequiredPresents) {
      break;
    }
    ++house;
  }
  std::cout << house << "\n";

  {
    std::vector<int64_t> houses;
    for (int64_t elf = 1;; ++elf) {
      auto const presents = elf * 11;
      for (int i = 1; i <= 50; ++i) {
        auto const house = elf * i;
        if (houses.size() <= house) {
          houses.resize(house+1, 0);
        }
        houses[house] += presents;
      }

      if (houses[elf] >= kRequiredPresents) {
        std::cout << elf << "\n";
        break;
      }
    }
  }

  return 0;
}
