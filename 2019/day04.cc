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

bool CheckForAdjacentDigits2(std::string const & s) {
  char c = s[0];
  int count = 1;
  for (int i = 1; i < 6; ++i) {
    if (s[i] == c) {
      ++count;
    } else if (count == 2) {
      return true;
    } else {
      c = s[i];
      count = 1;
    }
  }
  return count == 2;
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
  int count2 = 0;
  for (int i = input_min; i <= input_max; ++i) {
    auto s = ConvertToString(i);
    if (!CheckForIncreasingSequence(s)) {
      continue;
    }
    if (CheckForAdjacentDigits(s)) {
      ++count;
    }
    if (CheckForAdjacentDigits2(s)) {
      ++count2;
    }
  }
  std::cout
    << count << "\n"
    << count2 << "\n";
}

