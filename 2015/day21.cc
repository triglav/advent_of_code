#include <iostream>
#include <string>
#include <vector>

#include "set_utils.h"

struct Entity {
  int hit_points;
  int damage;
  int armor;
};

struct Item {
  std::string name;
  int cost;
  int damage;
  int armor;
};

void DealDamage(Entity attacker, Entity *defender) {
  auto const damage = std::max(1, attacker.damage - defender->armor);
  defender->hit_points -= damage;
}

bool SimulateFight(Entity player, Entity boss) {
  while (true) {
    DealDamage(player, &boss);
    if (boss.hit_points <= 0) {
      break;
    }
    DealDamage(boss, &player);
    if (player.hit_points <= 0) {
      break;
    }
  }
  return player.hit_points > 0;
}

int EquipItem(Entity *player, Item const &item) {
  player->damage += item.damage;
  player->armor += item.armor;
  return item.cost;
}

int main() {
  Entity boss{100, 8, 2};
  Entity player0{100, 0, 0};

  std::vector<Item> weapons_in_shop{
      {"Dagger", 8, 4, 0},     {"Shortsword", 10, 5, 0},
      {"Warhammer", 25, 6, 0}, {"Longsword", 40, 7, 0},
      {"Greataxe", 74, 8, 0},
  };

  std::vector<Item> armor_in_shop{
      {"Leather", 13, 0, 1},    {"Chainmail", 31, 0, 2},
      {"Splintmail", 53, 0, 3}, {"Bandedmail", 75, 0, 4},
      {"Platemail", 102, 0, 5},
  };

  std::vector<Item> rings_in_shop{
      {"Damage +1", 25, 1, 0},  {"Damage +2", 50, 2, 0},
      {"Damage +3", 100, 3, 0}, {"Defense +1", 20, 0, 1},
      {"Defense +2", 40, 0, 2}, {"Defense +3", 80, 0, 3},
  };

  auto const armor = With(armor_in_shop, {"Naked", 0, 0, 0});
  auto rings = GetCombinations(rings_in_shop, 0, false);
  {
    auto const rings1 = GetCombinations(rings_in_shop, 1, false);
    std::move(rings1.begin(), rings1.end(), std::back_inserter(rings));
    auto const rings2 = GetCombinations(rings_in_shop, 2, false);
    std::move(rings2.begin(), rings2.end(), std::back_inserter(rings));
  }

  int min_cost = std::numeric_limits<int>::max();
  for (auto const &w : weapons_in_shop) {
    for (auto const &a : armor) {
      for (auto const &ring_option : rings) {
        auto player = player0;
        int cost = 0;
        cost += EquipItem(&player, w);
        cost += EquipItem(&player, a);
        for (auto const &ring : ring_option) {
          cost += EquipItem(&player, ring);
        }
        if (min_cost > cost && SimulateFight(player, boss)) {
          min_cost = cost;
        }
      }
    }
  }
  std::cout << min_cost << "\n";
  return 0;
}
