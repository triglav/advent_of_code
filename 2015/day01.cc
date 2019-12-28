#include <cassert>
#include <iostream>

int main() {
  int floor = 0;

  char c;
  while (std::cin >> c) {
    if (c == '(') {
      ++floor;
      continue;
    } else if (c == ')') {
      --floor;
      continue;
    }
    assert(false);
  }
  std::cout << floor << "\n";
  return 0;
}

