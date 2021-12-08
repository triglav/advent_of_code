#include <algorithm>
#include <array>
#include <cstdlib>
#include <iostream>
#include <limits>
#include <numeric>
#include <unordered_map>

int main() {
  // position, count
  std::unordered_map<int, int> crabs;

  std::string token;
  while (std::getline(std::cin, token, ',')) {
    auto const f = std::stoi(token);
    auto const it = crabs.find(f);
    auto const c = (it != crabs.end()) ? it->second : 0;
    crabs.insert_or_assign(f, c + 1);
  }

  auto const min_max = std::minmax_element(
      crabs.begin(), crabs.end(),
      [](auto const &a, auto const &b) { return a.first < b.first; });

  auto const count = min_max.second->first - min_max.first->first;

  uint64_t fuel = std::numeric_limits<uint64_t>::max();
  for (int i = 0; i < count; ++i) {
    uint64_t f = 0;
    for (auto const &[pos, count] : crabs) {
      f += std::abs(pos - i) * count;
    }
    fuel = std::min(fuel, f);
  }
  std::cout << fuel << "\n";
  return 0;
}
