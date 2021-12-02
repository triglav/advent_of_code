#include <algorithm>
#include <iostream>
#include <vector>

int main() {
  int pos = 0;
  int depth = 0;
  int aim = 0;

  std::string command;
  int amount;
  while (std::cin >> command >> amount) {
    if (command == "forward") {
      pos += amount;
      depth += amount * aim;
    } else if (command == "down") {
      aim += amount;
    } else if (command == "up") {
      aim -= amount;
    }
  }
  std::cout << pos * depth << "\n";
  return 0;
}
