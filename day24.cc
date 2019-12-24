#include <bitset>
#include <iostream>
#include <unordered_set>
#include <vector>

int const kLevelCount = 250;

auto const kDimension = 5;
using Generation = std::bitset<kDimension * kDimension>;
using RecursiveGeneration = std::vector<Generation>;

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

int RecursiveAdjacentBugsCount(RecursiveGeneration const &rg, int l, int x,
                               int y) {
  auto &g = rg[l];

  int r = 0;

  r += x == 0 && rg[l - 1][idx(1, 2)];
  r += x > 0 && g[idx(x - 1, y)];
  r += x < (kDimension - 1) && g[idx(x + 1, y)];
  r += x == (kDimension - 1) && rg[l - 1][idx(3, 2)];

  r += y == 0 && rg[l - 1][idx(2, 1)];
  r += y > 0 && g[idx(x, y - 1)];
  r += y < (kDimension - 1) && g[idx(x, y + 1)];
  r += y == (kDimension - 1) && rg[l - 1][idx(2, 3)];

  if (x == 2 && y == 1) {
    for (int x2 = 0; x2 < kDimension; ++x2) {
      r += rg[l + 1][idx(x2, 0)];
    }
  }
  if (x == 2 && y == 3) {
    for (int x2 = 0; x2 < kDimension; ++x2) {
      r += rg[l + 1][idx(x2, kDimension - 1)];
    }
  }
  if (x == 1 && y == 2) {
    for (int y2 = 0; y2 < kDimension; ++y2) {
      r += rg[l + 1][idx(0, y2)];
    }
  }
  if (x == 3 && y == 2) {
    for (int y2 = 0; y2 < kDimension; ++y2) {
      r += rg[l + 1][idx(kDimension - 1, y2)];
    }
  }

  return r;
}

RecursiveGeneration RecursiveStep(RecursiveGeneration const &rg0) {
  RecursiveGeneration rg1 = rg0;
  for (auto l = 1; l < kLevelCount - 1; ++l) {
    auto const &g0 = rg0[l];
    auto &g1 = rg1.at(l);

    for (int y = 0; y < kDimension; ++y) {
      for (int x = 0; x < kDimension; ++x) {
        if (x == 2 && y == 2) {
          continue;
        }

        auto const i = idx(x, y);
        auto const adjacent_count = RecursiveAdjacentBugsCount(rg0, l, x, y);
        if (g0.test(i)) {
          auto const survived = adjacent_count == 1;
          g1.set(i, survived);
        } else {
          auto const infested = adjacent_count == 1 || adjacent_count == 2;
          g1.set(i, infested);
        }
      }
    }
  }
  return rg1;
}

int CountBugs(RecursiveGeneration const &rg) {
  int count = 0;
  for (auto const &g : rg) {
    count += g.count();
  }
  return count;
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
  {
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
  }
  {
    RecursiveGeneration g(kLevelCount);
    g[kLevelCount / 2] = first_generation;
    for (int i = 0; i < 200; ++i) {
      g = RecursiveStep(g);
    }
    auto const bugs = CountBugs(g);
    std::cout << bugs << "\n";
  }
  return 0;
}
