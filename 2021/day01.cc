#include <iostream>

int main() {
  int depth0;
  int depth;
  std::cin >> depth0;
  int count = 0;
  while (std::cin >> depth) {
    if (depth > depth0) {
      ++count;
    }
    depth0 = depth;
  }
  std::cout << count << "\n";
  return 0;
}
