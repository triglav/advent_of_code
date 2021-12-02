#include <algorithm>
#include <iostream>
#include <limits>
#include <numeric>
#include <vector>
#include <cassert>

std::vector<std::vector<int>> foo(std::vector<int> used, std::vector<int> weights, int desired) {
  std::vector<std::vector<int>> r;
  for (auto i = weights.cbegin(); i != weights.cend(); ++i) {
    const auto w = *i;
    assert(i == weights.cbegin() || *(i - 1) < w);
    if (w > desired) {
      continue;
    }
    if (w == desired) {
      std::vector<int> r2(used);
      r2.push_back(w);
      r.push_back(r2);
      continue;
    }
    if (w < desired) {
      std::vector<int> used2(used);
      used2.push_back(w);

      std::vector<int> weights2;
      std::copy(weights.cbegin(), i, std::back_inserter(weights2));

      auto r2 = foo(used2, weights2, desired - w);
      std::copy(r2.cbegin(), r2.cend(), std::back_inserter(r));
      continue;
    }
  }
  return r;
}

int main() {
  std::vector<int> packages;

  int weight;
  int weight_sum = 0;
  while (std::cin >> weight) {
    weight_sum += weight;
    packages.push_back(weight);
  }
  int const desired_weight = weight_sum / 4;
  assert(weight_sum % 4 == 0);

  auto results = foo({}, packages, desired_weight);
  std::sort(results.begin(), results.end(), [](auto const & a, auto const & b) {
    return a.size() < b.size();
  });

  uint64_t entanglement = std::numeric_limits<uint64_t>::max();
  for (auto i = results.begin(); i != results.end(); ++i) {
    auto const & r = *i;
    auto const e = std::accumulate(r.begin(), r.end(), 1ULL, [](auto a, auto b) { return a * b; });
    if (e < entanglement) {
      entanglement = e;
    }
    if (i != results.begin() && (i-1)->size() < r.size()) {
      break;
    }
  }

  std::cout << entanglement << "\n";
  return 0;
}
