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

std::pair<int, int>
CalculateCookieScore(std::vector<Ingredient const *> const &ingredients) {
  Ingredient mixture{0, 0, 0, 0, 0};
  for (auto i : ingredients) {
    mixture.capacity += i->capacity;
    mixture.durability += i->durability;
    mixture.flavor += i->flavor;
    mixture.texture += i->texture;
    mixture.calories += i->calories;
  }
  mixture.capacity = std::max(0, mixture.capacity);
  mixture.durability = std::max(0, mixture.durability);
  mixture.flavor = std::max(0, mixture.flavor);
  mixture.texture = std::max(0, mixture.texture);

  auto const score =
      mixture.capacity * mixture.durability * mixture.flavor * mixture.texture;
  return {score, ((mixture.calories == 500) ? score : 0)};
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

  int max_score1 = 0;
  int max_score2 = 0;
  for (auto const &c : combinations) {
    auto const [score1, score2] = CalculateCookieScore(c);
    max_score1 = std::max(max_score1, score1);
    max_score2 = std::max(max_score2, score2);
  }
  std::cout << max_score1 << "\n" << max_score2 << "\n";
  return 0;
}
