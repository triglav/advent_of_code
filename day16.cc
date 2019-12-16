#include <iostream>
#include <string>
#include <vector>

using Pattern = std::vector<int>;

std::vector<Pattern> BuildPatterns(Pattern const &base_pattern,
                                   size_t const size) {
  std::vector<Pattern> patterns{size};
  for (int i = 0; i < size; ++i) {
    auto &p = patterns[i];
    auto x = 0;
    auto bi = 0;
    while (x <= size) {
      for (int i2 = 0; i2 <= i && x <= size; ++i2) {
        if (x > 0) {
          p.push_back(base_pattern[bi]);
        }
        ++x;
      }
      bi = (bi + 1) % base_pattern.size();
    }
  }
  return patterns;
}

std::vector<int> DoFFT(std::vector<int> const &input_list, int phase,
                       std::vector<Pattern> const &patterns) {
  std::vector<int> output_list;
  for (int i = 0; i < input_list.size(); ++i) {
    auto const &p = patterns[i];
    int64_t r = 0;
    for (int i2 = 0; i2 < input_list.size(); ++i2) {
      r += input_list[i2] * p[i2];
    }
    output_list.push_back(std::abs(r) % 10);
  }
  return output_list;
}

int main() {
  std::vector<int> elements;
  {
    std::string line;
    std::getline(std::cin, line);
    for (auto c : line) {
      elements.push_back(c - '0');
    }
  }

  auto patterns = BuildPatterns({0, 1, 0, -1}, elements.size());

  auto l = elements;
  for (int i = 1; i <= 100; ++i) {
    l = DoFFT(l, i, patterns);
  }
  for (int i = 0; i < 8; ++i) {
    std::cout << l[i];
  }
  std::cout << "\n";
  return 0;
}
