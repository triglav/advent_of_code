#include <cassert>
#include <iostream>
#include <string>

int CountActualChars(std::string const &s) {
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

std::string EscapeString(std::string const &s) {
  std::string s2 = "\"";
  for (int i = 0; i < s.size(); ++i) {
    auto c = s[i];
    if (c == '"' || c == '\\') {
      s2.push_back('\\');
    }
    s2.push_back(c);
  }
  s2.push_back('"');
  return s2;
}

int main() {
  int code_chars_count = 0;
  int chars_count = 0;
  int escaped_code_chars_count = 0;

  std::string line;
  while (std::getline(std::cin, line)) {
    code_chars_count += line.size();
    chars_count += CountActualChars(line);

    auto const escaped_line = EscapeString(line);
    escaped_code_chars_count += escaped_line.size();
  }

  std::cout << code_chars_count - chars_count << "\n";
  std::cout << escaped_code_chars_count - code_chars_count << "\n";

  return 0;
}
