#include <iostream>
#include <numeric>
#include <string>
#include <unordered_map>

#include "set_utils.h"

int main() {
  int const kLiters = 150;
  std::vector<int> containers;

  int c;
  while (std::cin >> c) {
    containers.push_back(c);
  }

  std::vector<std::vector<int>> container_combinations;
  for (int i = 1; i < containers.size(); ++i) {
    auto c = GetCombinations(containers, i, false);
    std::move(c.begin(), c.end(), std::back_inserter(container_combinations));
  }

  int count = 0;
  int min_container_count = std::numeric_limits<int>::max();
  int ways_count = 0;
  for (auto const &c : container_combinations) {
    auto const sum = std::accumulate(c.begin(), c.end(), 0);
    if (sum == kLiters) {
      ++count;
      if (c.size() <= min_container_count) {
        if (c.size() < min_container_count) {
          min_container_count = c.size();
          ways_count = 0;
        }
        ++ways_count;
      }
    }
  }
  std::cout << count << "\n" << ways_count << "\n";
  return 0;
}
