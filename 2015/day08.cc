#include <cassert>
#include <iostream>
#include <string>

int CountActualChars(std::string const & s) {
  int chars_count = 0;
  for (int i = 1; i < s.size() - 1; ++i) {
    if (s[i] == '\\') {
      if (s[i + 1] == '\\' || s[i + 1] == '"') {
        ++i;
      } else if (s[i + 1] == 'x') {
        i += 3;
      }
    }
    ++chars_count;
    assert(i < s.size() - 1);
  }
  return chars_count;
}

int main() {
  int code_chars_count = 0;
  int chars_count = 0;

  std::string line;
  while (std::getline(std::cin, line)) {
    code_chars_count += line.size();
    chars_count += CountActualChars(line);
  }

  std::cout << code_chars_count - chars_count << "\n";

  return 0;
}
