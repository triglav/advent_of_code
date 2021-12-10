#include <algorithm>
#include <array>
#include <iostream>
#include <iterator>
#include <string>
#include <tuple>
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
    std::sort(s.patterns[i].begin(), s.patterns[i].end());
  }
  input.ignore(42, '|');
  for (int i = 0; i < 4; ++i) {
    input >> s.digits[i];
    std::sort(s.digits[i].begin(), s.digits[i].end());
  }
  return input;
}

std::vector<std::string> FindPatternsByLength(Screen const &s, int length) {
  std::vector<std::string> r;
  std::copy_if(s.patterns.begin(), s.patterns.end(), std::back_inserter(r),
               [length](auto const &p) { return p.size() == length; });
  return r;
}

std::string PatternDifference(std::string const &a, std::string const &b) {
  std::string r = "";
  std::copy_if(a.begin(), a.end(), std::back_inserter(r),
               [&b](auto c) { return b.find(c) == std::string::npos; });
  return r;
}

std::tuple<std::string, std::vector<std::string>, char, char>
FindPattern6And09AndSignalCF(std::vector<std::string> const &pattern069,
                             std::string const &pattern1) {
  std::string pattern6;
  std::vector<std::string> pattern09;
  char signal_c;
  char signal_f;
  for (auto const &p : pattern069) {
    for (auto s : pattern1) {
      if (p.find(s) == std::string::npos) {
        pattern6 = p;
        signal_c = s;
        signal_f = ((pattern1[0] == s) ? pattern1[1] : pattern1[0]);
        break;
      }
    }
  }
  for (auto const &p : pattern069) {
    if (p != pattern6) {
      pattern09.push_back(p);
    }
  }
  return std::make_tuple(pattern6, pattern09, signal_c, signal_f);
}

std::tuple<std::string, std::vector<std::string>>
FindPattern5And23(std::vector<std::string> const &pattern235, char signal_c) {
  std::string pattern5;
  std::vector<std::string> pattern23;
  for (auto const &p : pattern235) {
    if (p.find(signal_c) == std::string::npos) {
      pattern5 = p;
    } else {
      pattern23.push_back(p);
    }
  }
  return std::make_tuple(pattern5, pattern23);
}

std::tuple<std::string, std::string>
FindPattern2And3(std::vector<std::string> const &pattern23, char signal_f) {
  if (pattern23[0].find(signal_f) == std::string::npos) {
    return std::make_tuple(pattern23[0], pattern23[1]);
  }
  return std::make_tuple(pattern23[1], pattern23[0]);
}

std::tuple<std::string, std::string>
FindPattern0And9(std::vector<std::string> const &pattern09,
                 std::string const &pattern3) {
  auto d = PatternDifference(pattern09[0], pattern3);
  if (d.size() == 2) {
    return std::make_tuple(pattern09[0], pattern09[1]);
  }
  return std::make_tuple(pattern09[1], pattern09[0]);
}

std::array<std::string, 10> SolveSignals(Screen const &s) {
  auto pattern1 = FindPatternsByLength(s, 2)[0];
  auto pattern4 = FindPatternsByLength(s, 4)[0];
  auto pattern7 = FindPatternsByLength(s, 3)[0];
  auto pattern8 = FindPatternsByLength(s, 7)[0];

  auto pattern235 = FindPatternsByLength(s, 5);
  auto pattern069 = FindPatternsByLength(s, 6);

  auto const signal_a = PatternDifference(pattern7, pattern1)[0];
  auto const [pattern6, pattern09, signal_c, signal_f] =
      FindPattern6And09AndSignalCF(pattern069, pattern1);
  auto const [pattern5, pattern23] = FindPattern5And23(pattern235, signal_c);
  auto const [pattern2, pattern3] = FindPattern2And3(pattern23, signal_f);
  auto const [pattern0, pattern9] = FindPattern0And9(pattern09, pattern3);

  return {
      pattern0, pattern1, pattern2, pattern3, pattern4,
      pattern5, pattern6, pattern7, pattern8, pattern9,
  };
}

int main() {
  std::array<int, 10> digit_count = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0};

  int sum = 0;
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
    auto const digits = SolveSignals(s);
    int value = 0;
    for (auto const &d : s.digits) {
      for (int i = 0; i < 10; ++i) {
        if (d == digits[i]) {
          value *= 10;
          value += i;
          break;
        }
      }
    }
    sum += value;
  }
  auto const count1478 =
      digit_count[1] + digit_count[4] + digit_count[7] + digit_count[8];
  std::cout << count1478 << "\n";
  std::cout << sum << "\n";
  return 0;
}
