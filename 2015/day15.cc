#include <algorithm>
#include <iostream>
#include <string>

#include "set_utils.h"
#include "string_utils.h"

struct Ingredient {
  int capacity;
  int durability;
  int flavor;
  int texture;
  int calories;
};

int CalculateCookieScore(std::vector<Ingredient const *> const &ingredients) {
  Ingredient mixture{0, 0, 0, 0, 0};
  for (auto i : ingredients) {
    mixture.capacity += i->capacity;
    mixture.durability += i->durability;
    mixture.flavor += i->flavor;
    mixture.texture += i->texture;
  }
  mixture.capacity = std::max(0, mixture.capacity);
  mixture.durability = std::max(0, mixture.durability);
  mixture.flavor = std::max(0, mixture.flavor);
  mixture.texture = std::max(0, mixture.texture);
  return mixture.capacity * mixture.durability * mixture.flavor *
         mixture.texture;
}

int main() {
  std::vector<Ingredient> ingredients;

  std::string line;
  while (std::getline(std::cin, line)) {
    auto const t = SplitString(line);

    auto const i = Ingredient{
        sv2number<int>(t[2]), sv2number<int>(t[4]),  sv2number<int>(t[6]),
        sv2number<int>(t[8]), sv2number<int>(t[10]),
    };

    ingredients.emplace_back(std::move(i));
  }

  std::vector<Ingredient const *> samples;
  for (auto const &i : ingredients) {
    samples.push_back(&i);
  }
  auto const combinations = GetCombinations(samples, 100);

  int max_score = 0;
  for (auto const &c : combinations) {
    auto const score = CalculateCookieScore(c);
    max_score = std::max(max_score, score);
  }
  std::cout << max_score << "\n";
  return 0;
}
