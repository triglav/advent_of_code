#include <cassert>
#include <iostream>
#include <optional>

int main() {
  int floor = 0;
  std::optional<int> position;

  int current_position = 0;
  char c;
  while (std::cin >> c) {
    ++current_position;
    assert(c == '(' || c == ')');
    if (c == '(') {
      ++floor;
    } else {
      --floor;
    }
    if (floor == -1 && !position.has_value()) {
      position = current_position;
    }
  }
  std::cout << floor << "\n";
  assert(position.has_value());
  std::cout << *position << "\n";
  return 0;
}
