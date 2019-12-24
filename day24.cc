#include <bitset>
#include <iostream>
#include <unordered_set>

auto const kDimension = 5;
using Generation = std::bitset<kDimension * kDimension>;

inline int idx(int x, int y) { return y * kDimension + x; }

int AdjacentBugsCount(Generation const &g, int x, int y) {
  int r = 0;

  r += x > 0 && g[idx(x - 1, y)];
  r += x < (kDimension - 1) && g[idx(x + 1, y)];

  r += y > 0 && g[idx(x, y - 1)];
  r += y < (kDimension - 1) && g[idx(x, y + 1)];

  return r;
}

Generation Step(Generation const &g0) {
  Generation g1;
  for (int y = 0; y < kDimension; ++y) {
    for (int x = 0; x < kDimension; ++x) {
      auto const i = idx(x, y);
      auto const adjacent_count = AdjacentBugsCount(g0, x, y);
      if (g0.test(i)) {
        auto const survived = adjacent_count == 1;
        g1.set(i, survived);
      } else {
        auto const infested = adjacent_count == 1 || adjacent_count == 2;
        g1.set(i, infested);
      }
    }
  }
  return g1;
}

int main() {
  Generation first_generation;
  {
    int i = 0;
    for (std::string line; std::getline(std::cin, line);) {
      for (auto c : line) {
        first_generation.set(i++, c == '#');
      }
    }
  }
  std::unordered_set<unsigned long> previous_ratings;
  auto g = first_generation;
  while (true) {
    auto const rating = g.to_ulong();
    if (previous_ratings.find(rating) != previous_ratings.end()) {
      std::cout << rating << "\n";
      break;
    }
    previous_ratings.insert(rating);
    g = Step(g);
  }
  return 0;
}
