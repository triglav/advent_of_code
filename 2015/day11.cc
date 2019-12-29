#include <cassert>
#include <iostream>
#include <string>

std::string Increment(std::string p) {
  for (int i = p.size() - 1; i >= 0; --i) {
    auto c = p[i];
    if (c != 'z') {
      p[i] += 1;
      break;
    }
    p[i] = 'a';
    assert(i > 0);
  }
  return p;
}

bool CheckRule1(std::string_view p) {
  for (int i = 0; i < p.size() - 2; ++i) {
    if (p[i] < 'y' && p[i] == p[i + 1] - 1 && p[i] == p[i + 2] - 2) {
      return true;
    }
  }
  return false;
}

bool CheckRule2(std::string_view p) {
  return p.find_first_of("iol") == std::string::npos;
}

bool CheckRule3(std::string_view p) {
  int pair_count = 0;
  for (int i = 0; i < p.size() - 1; ++i) {
    if (p[i] == p[i + 1]) {
      ++pair_count;
      ++i;
    }
  }
  return pair_count >= 2;
}

std::string FindNextPassword(std::string const &p0) {
  auto p = Increment(p0);
  while (!CheckRule1(p) || !CheckRule2(p) || !CheckRule3(p)) {
    p = Increment(p);
  }
  return p;
}

int main() {
  std::string p0 = "hxbxwxba";
  auto p = FindNextPassword(p0);
  std::cout << p << "\n";

  auto p2 = FindNextPassword(p);
  std::cout << p2 << "\n";
  return 0;
}
