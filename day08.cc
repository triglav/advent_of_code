#include <array>
#include <iostream>
#include <optional>

using Digits = std::array<int, 3>;

std::optional<Digits> Parse(int const width, int const height) {
  int count = width * height;
  Digits digits = {0, 0, 0};

  char c;
  while (count > 0 && std::cin >> c) {
    ++digits[c - '0'];
    --count;
  }
  if (count > 0) {
    return std::nullopt;
  }
  return digits;
};

int main() {
  Digits max_digits = {std::numeric_limits<int>::max(), 0, 0};
  while (true) {
    auto r = Parse(25, 6);
    if (!r) {
      break;
    }
    auto const & digits = *r;
    if (max_digits[0] > digits[0]) {
      max_digits = digits;
    }
  }
  std::cout << max_digits[1] * max_digits[2] << "\n";
  return 0;
}

