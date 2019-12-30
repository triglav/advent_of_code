#include <cmath>
#include <iostream>

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
  return 0;
}
