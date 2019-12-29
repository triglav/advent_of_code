#include <iostream>
#include <vector>

#include "json.hpp"

using json = nlohmann::json;

int64_t Sum(json const &j) {
  int64_t r = 0;
  for (auto const &x : j) {
    if (x.is_number()) {
      r += x.get<int64_t>();
    } else if (x.is_object() || x.is_array()) {
      r += Sum(x);
    }
  }
  return r;
}

int main() {
  json j;
  std::cin >> j;

  std::cout << Sum(j) << "\n";

  return 0;
}
