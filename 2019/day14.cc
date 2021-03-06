#include <deque>
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

using Name = std::string;
struct Part {
  Name name;
  int64_t quantity;
};

Part ParseChemicalAndQuantity(std::string const &token) {
  auto const d = token.find_first_of(" ");
  auto t0 = token.substr(0, d);
  auto t1 = token.substr(d + 1);
  return {t1, std::stol(t0)};
}

std::vector<Part> ParseChemicals(std::string const &token) {
  std::vector<Part> v;

  std::stringstream ss(token);
  std::string token2;
  while (std::getline(ss, token2, ',')) {
    v.push_back(ParseChemicalAndQuantity(token2));
    ss.get();
  }
  return v;
}

struct Recipe {
  int64_t quantity;
  std::vector<Part> parts;
};

int64_t MakeFuel(int64_t fuel_count,
                  std::unordered_map<Name, Recipe> const &recipes) {
  int64_t ore_count = 0;
  std::deque<Part> to_check;
  to_check.push_back({"FUEL", fuel_count});

  std::unordered_map<Name, int64_t> spare_chemicals;

  while (!to_check.empty()) {
    auto need = to_check.front();
    to_check.pop_front();

    if (need.name == "ORE") {
      ore_count += need.quantity;
      continue;
    }

    auto const it = spare_chemicals.find(need.name);
    auto const spare = ((it != spare_chemicals.end()) ? it->second : 0);
    auto const to_create = std::max<int64_t>(0, need.quantity - spare);
    spare_chemicals[need.name] = std::max<int64_t>(0, spare - need.quantity);
    if (to_create <= 0) {
      continue;
    }

    auto const &r = recipes.at(need.name);
    auto const x1 = to_create / r.quantity;
    auto const x2 = to_create % r.quantity;
    auto const x = x1 + ((x2 > 0) ? 1 : 0);
    auto const created = x * r.quantity;
    spare_chemicals[need.name] += created - to_create;

    for (auto const &p1 : r.parts) {
      auto const required = x * p1.quantity;
      to_check.push_back({p1.name, required});
    }
  }
  return ore_count;
}

int main() {
  std::unordered_map<Name, Recipe> recipes;

  std::string line;
  while (std::getline(std::cin, line)) {
    auto const d = line.find_first_of("=");

    auto t0 = line.substr(0, d - 1);
    auto input_chemicals = ParseChemicals(t0);

    auto t1 = line.substr(d + 3);
    auto const p0 = ParseChemicalAndQuantity(t1);
    assert(recipes.find(p0.name) == recipes.end());
    recipes.emplace(p0.name, Recipe{p0.quantity, input_chemicals});
  }

  int64_t ore_count = MakeFuel(1, recipes);
  std::cout << ore_count << "\n";

  int64_t const kMinedOre = 1000000000000;
  int64_t l = 0;
  int64_t r = kMinedOre;
  int64_t result;
  while (l <= r) {
    int64_t const m = std::floor((r + l) / 2);
    int64_t const ore = MakeFuel(m, recipes);
    if (ore < kMinedOre) {
      result = m;
      l = m + 1;
    } else if (ore >= kMinedOre) {
      r = m - 1;
    }
  }
  std::cout << result << "\n";
  return 0;
}
