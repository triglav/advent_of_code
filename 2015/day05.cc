#include <iostream>
#include <string>

std::string const vowels = "aeiou";

bool IsNice(std::string const &s) {
  int vowels_count = 0;
  bool twice = false;
  for (int i = 0; i < s.size(); ++i) {
    auto c = s[i];
    if (vowels.find(c) != std::string::npos) {
      ++vowels_count;
    }
    if (i > 0) {
      auto c2 = s[i - 1];
      if ((c == 'b' && c2 == 'a') || (c == 'd' && c2 == 'c') ||
          (c == 'q' && c2 == 'p') || (c == 'y' && c2 == 'x')) {
        return false;
      }
      if (c == c2) {
        twice = true;
      }
    }
  }
  return vowels_count >= 3 && twice;
}

int main() {
  int count = 0;

  std::string line;
  while (std::getline(std::cin, line)) {
    if (IsNice(line)) {
      ++count;
    }
  }
  std::cout << count << "\n";
  return 0;
}
