#include <iostream>
#include <string>
#include <vector>

using Pattern = std::vector<int>;

int GetPatternValue(Pattern const &base_pattern, int row_idx, int element_idx) {
  auto const y = row_idx + 1;
  auto const t = 4 * y;
  auto const x = (element_idx + 1) % t;
  if (x >= 2 * y) {
    if (x >= 3 * y) {
      return base_pattern[3];
    }
    return base_pattern[2];
  } else if (x >= y) {
    return base_pattern[1];
  }
  return base_pattern[0];
}

std::vector<int> DoFFT(std::vector<int> const &input_list, int phase,
                       Pattern const &base_pattern) {
  std::vector<int> output_list;
  for (int i = 0; i < input_list.size(); ++i) {
    int64_t r = 0;
    for (int i2 = 0; i2 < input_list.size(); ++i2) {
      r += input_list[i2] * GetPatternValue(base_pattern, i, i2);
    }
    output_list.push_back(std::abs(r) % 10);
  }
  return output_list;
}

std::vector<int> DoFFT_partial(std::vector<int> const &input_list, int phase,
                               Pattern const &base_pattern, int64_t offset) {
  std::vector<int> output_list;
  output_list.resize(input_list.size());

  assert(offset > input_list.size() / 2);

  int64_t s = 0;
  for (int64_t i = offset; i < input_list.size(); ++i) {
    s += input_list[i];
  }
  for (int64_t i = offset; i < input_list.size(); ++i) {
    auto r = s;
    s -= input_list[i];
    output_list[i] = std::abs(r) % 10;
  }
  return output_list;
}

int main() {
  std::vector<int> base_elements;
  {
    std::string line;
    std::getline(std::cin, line);
    for (auto c : line) {
      base_elements.push_back(c - '0');
    }
  }
  int64_t offset = 0;
  for (int i = 0; i < 7; ++i) {
    auto const d = base_elements[i];
    offset = offset * 10 + d;
  }

  Pattern base_pattern{0, 1, 0, -1};
  {
    auto l = base_elements;
    for (int i = 1; i <= 100; ++i) {
      l = DoFFT(l, i, base_pattern);
    }
    for (int i = 0; i < 8; ++i) {
      std::cout << l[i];
    }
    std::cout << "\n";
  }

  std::vector<int> elements;
  for (int i = 0; i < 10000; ++i) {
    std::copy(base_elements.begin(), base_elements.end(),
              std::back_inserter(elements));
  }

  auto l = elements;
  for (int i = 1; i <= 100; ++i) {
    l = DoFFT_partial(l, i, base_pattern, offset);
  }
  for (auto i = offset; i < offset + 8; ++i) {
    std::cout << l[i];
  }
  std::cout << "\n";
  return 0;
}
