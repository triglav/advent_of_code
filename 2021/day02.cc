#include <algorithm>
#include <iostream>
#include <vector>

int main() {
  int pos = 0;
  int depth = 0;

  std::string command;
  int amount;
  while (std::cin >> command >> amount) {
    if (command == "forward") {
      pos += amount;
    } else if (command == "down") {
      depth += amount;
    } else if (command == "up") {
      depth -= amount;
    }
  }
  std::cout << pos * depth << "\n";
  return 0;
}
