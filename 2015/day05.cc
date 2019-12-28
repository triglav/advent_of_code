#include <iostream>
#include <string>
#include <unordered_map>

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

bool IsNice2a(std::string const &s) {
  std::unordered_map<std::string, int> pairs;
  for (int i = 1; i < s.size(); ++i) {
    auto const c0 = s[i-1];
    auto const c1 = s[i];
    std::string const s2 = {c0, c1};

    auto const it = pairs.find(s2);
    if (it == pairs.end()) {
      pairs.insert({s2, i});
      continue;
    }
    if (i - it->second >= 2) {
      return true;
    }
  }
  return false;
}

bool IsNice2b(std::string const &s) {
  for (int i = 0; i < s.size() - 2; ++i) {
    if (s[i] == s[i+2] && s[i] != s[i+1]) {
      return true;
    }
  }
  return false;
}

bool IsNice2(std::string const &s) {
  return IsNice2b(s) && IsNice2a(s);
}

int main() {
  int count = 0;
  int count2 = 0;

  std::string line;
  while (std::getline(std::cin, line)) {
    if (IsNice(line)) {
      ++count;
    }
    if (IsNice2(line)) {
      ++count2;
    }
  }
  std::cout << count << "\n" << count2 << "\n";
  return 0;
}
