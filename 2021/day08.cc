#include <array>
#include <iostream>
#include <string>
#include <vector>

struct Screen {
  std::array<std::string, 10> patterns;
  std::array<std::string, 4> digits;
};

std::istream &operator>>(std::istream &input, Screen &s) {
  if (!input.good()) {
    return input;
  }

  for (int i = 0; i < 10; ++i) {
    input >> s.patterns[i];
  }
  input.ignore(42, '|');
  for (int i = 0; i < 4; ++i) {
    input >> s.digits[i];
  }
  return input;
}

int main() {
  std::array<int, 10> digit_count = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0};

  Screen s;
  while (std::cin >> s) {
    for (auto const &d : s.digits) {
      switch (d.size()) {
      case 2:
        digit_count[1] += 1;
        break;
      case 4:
        digit_count[4] += 1;
        break;
      case 3:
        digit_count[7] += 1;
        break;
      case 7:
        digit_count[8] += 1;
        break;
      }
    }
  }
  auto const count1478 =
      digit_count[1] + digit_count[4] + digit_count[7] + digit_count[8];
  std::cout << count1478 << "\n";
  return 0;
}
