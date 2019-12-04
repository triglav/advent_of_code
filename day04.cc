#include <iostream>
#include <sstream>
#include <string>

std::string ConvertToString(int x) {
  std::stringstream ss;
  ss << x;
  return ss.str();
}

bool CheckForAdjacentDigits(std::string const & s) {
  for (int i = 0; i < 5; ++i) {
    if (s[i] == s[i+1]) {
      return true;
    }
  }
  return false;
}

bool CheckForIncreasingSequence(std::string const & s) {
  for (int i = 0; i < 5; ++i) {
    if (s[i] > s[i+1]) {
      return false;
    }
  }
  return true;
}

int main() {
  int const input_min = 156218;
  int const input_max = 652527;

  int count = 0;
  for (int i = input_min; i <= input_max; ++i) {
    auto s = ConvertToString(i);
    if (!CheckForAdjacentDigits(s)) {
      continue;
    }
    if (!CheckForIncreasingSequence(s)) {
      continue;
    }
    ++count;
  }
  std::cout << count << "\n";
}

