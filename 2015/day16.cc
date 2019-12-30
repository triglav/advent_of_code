#include <iostream>
#include <string>
#include <unordered_map>

#include "string_utils.h"

bool Check(std::unordered_map<std::string_view, int> const &known_facts,
           std::vector<std::string_view> const &tokens) {
  for (int i = 2; i < tokens.size(); i += 2) {
    auto const thing = Trim(tokens[i], ":,");
    auto const it = known_facts.find(thing);
    if (it == known_facts.end()) {
      continue;
    }
    auto const count = sv2number<int>(Trim(tokens[i + 1], ":,"));
    if (it->second != count) {
      return false;
    }
  }
  return true;
}

int main() {
  std::unordered_map<std::string_view, int> known_facts;
  known_facts.emplace("children", 3);
  known_facts.emplace("cats", 7);
  known_facts.emplace("samoyeds", 2);
  known_facts.emplace("pomeranians", 3);
  known_facts.emplace("akitas", 0);
  known_facts.emplace("vizslas", 0);
  known_facts.emplace("goldfish", 5);
  known_facts.emplace("trees", 3);
  known_facts.emplace("cars", 2);
  known_facts.emplace("perfumes", 1);

  std::string line;
  while (std::getline(std::cin, line)) {
    auto const t = SplitString(line);

    if (Check(known_facts, t)) {
      auto const idx = sv2number<int>(Trim(t[1], ":,"));
      std::cout << idx << "\n";
    }
  }
  return 0;
}
