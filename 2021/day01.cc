#include <algorithm>
#include <iostream>
#include <vector>

int main() {
  std::vector<int> depth;
  for (int i = 0; i < 3; ++i) {
    int d;
    std::cin >> d;
    depth.push_back(d);
  }
  depth.resize(4);
  int d;
  int count = 0;
  while (std::cin >> depth[3]) {
    if (depth[3] > depth[0]) {
      ++count;
    }
    std::rotate(depth.begin(), depth.begin() + 1, depth.end());
  }
  std::cout << count << "\n";
  return 0;
}
