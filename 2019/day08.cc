#include <array>
#include <iostream>
#include <optional>

int const kWidth = 25;
int const kHeight = 6;

using Digits = std::array<int, 3>;
using Image = std::array<int, kWidth * kHeight>;

std::optional<Digits> Parse(Image *image) {
  Digits digits = {0, 0, 0};

  char c;
  for (int y = 0; y < kHeight; ++y) {
    for (int x = 0; x < kWidth; ++x) {
      std::cin >> c;
      if (!std::cin.good()) {
        return std::nullopt;
      }
      ++digits[c - '0'];
      auto &pixel = (*image)[y * kWidth + x];
      if (pixel == 2) {
        pixel = c - '0';
      }
    }
  }
  return digits;
};

int main() {
  Digits max_digits = {std::numeric_limits<int>::max(), 0, 0};
  Image image;
  image.fill(2);
  while (true) {
    auto r = Parse(&image);
    if (!r) {
      break;
    }
    auto const &digits = *r;
    if (max_digits[0] > digits[0]) {
      max_digits = digits;
    }
  }
  std::cout << max_digits[1] * max_digits[2] << "\n";

  for (int y = 0; y < kHeight; ++y) {
    for (int x = 0; x < kWidth; ++x) {
      auto pixel = image[y * kWidth + x];
      if (pixel == 1) {
        std::cout << "X";
      } else {
        std::cout << " ";
      }
    }
    std::cout << "\n";
  }
  return 0;
}

