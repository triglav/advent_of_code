#include <iostream>
#include <string>

std::string Transform(std::string const &s) {
  std::string s2;

  char c;
  int count = 0;
  for (int i = 0; i < s.size(); ++i) {
    if (i > 0 && c != s[i]) {
      s2.append(std::to_string(count));
      s2.push_back(c);

      count = 0;
    }
    c = s[i];
    ++count;
  }
  s2.append(std::to_string(count));
  s2.push_back(c);
  return s2;
}

int main() {
  std::string const s0 = "1113122113";

  auto s = s0;
  for (int i = 0; i < 40; ++i) {
    s = Transform(s);
  }
  std::cout << s.size() << "\n";
  return 0;
}
