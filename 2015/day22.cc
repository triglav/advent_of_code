#include <cassert>
#include <deque>
#include <iostream>
#include <string>
#include <vector>

#include "set_utils.h"

struct Boss {
  int hit_points;
  int damage;
};

struct Player {
  int hit_points;
  int damage;
  int armor;
  int mana;
};

enum class EffectType {
  Shield,
  Poison,
  Recharge,
};

struct Effect {
  EffectType type;
  int duration;
  int value;
};

struct Spell {
  std::string_view name;
  int mana_cost;
  int damage;
  int heal;
  std::optional<Effect> effect;
};

struct FightContext {
  Player player;
  Boss boss;
  std::deque<Spell> spell_queue;
  std::vector<Effect> active_effects;
  int mana_spent{0};
  std::vector<std::string_view> spell_history;
};

void DealDamage(Boss boss, Player *player) {
  auto const damage = std::max(1, boss.damage - player->armor);
  player->hit_points -= damage;
}

void CastSpell(FightContext *c) {
  assert(!c->spell_queue.empty());
  auto const spell = c->spell_queue.front();
  c->spell_queue.pop_front();
  assert(c->player.mana >= spell.mana_cost);

  c->spell_history.push_back(spell.name);

  c->mana_spent += spell.mana_cost;
  c->player.mana -= spell.mana_cost;
  c->boss.hit_points -= spell.damage;
  c->player.hit_points += spell.heal;
  if (spell.effect.has_value()) {
    assert(std::none_of(
        c->active_effects.begin(), c->active_effects.end(),
        [&spell](Effect const &e) { return e.type == spell.effect->type; }));
    c->active_effects.push_back(*spell.effect);
    if (spell.effect->type == EffectType::Shield) {
      c->player.armor += spell.effect->value;
    }
  }
}

void UpdateActiveEffects(FightContext *c) {
  for (auto it = c->active_effects.begin(); it != c->active_effects.end();) {
    --it->duration;
    switch (it->type) {
    case EffectType::Shield:
      if (it->duration <= 0) {
        c->player.armor -= it->value;
      }
      break;
    case EffectType::Poison:
      c->boss.hit_points -= it->value;
      break;
    case EffectType::Recharge:
      c->player.mana += it->value;
      break;
    }
    if (it->duration <= 0) {
      it = c->active_effects.erase(it);
    } else {
      ++it;
    }
  }
}

void SimulateFightTurn(FightContext *c) {
  UpdateActiveEffects(c);
  if (c->boss.hit_points <= 0) {
    return;
  }
  CastSpell(c);
  if (c->boss.hit_points <= 0) {
    return;
  }

  UpdateActiveEffects(c);
  if (c->boss.hit_points <= 0) {
    return;
  }
  DealDamage(c->boss, &c->player);
}

std::vector<size_t> ListAvailableSpells(FightContext const &c,
                                        std::vector<Spell> const &all_spells) {
  std::vector<size_t> r;
  for (size_t i = 0; i < all_spells.size(); ++i) {
    auto const &s = all_spells[i];
    if (s.mana_cost > c.player.mana) {
      continue;
    }
    if (s.effect.has_value() &&
        std::any_of(c.active_effects.begin(), c.active_effects.end(),
                    [&s](Effect const &e) {
                      return e.type == s.effect->type && e.duration > 1;
                    })) {
      continue;
    }
    r.push_back(i);
  }
  return r;
}

int Simulate(FightContext const &c0, std::vector<Spell> const &all_spells) {
  int min_mana = std::numeric_limits<int>::max();

  std::deque<FightContext> to_check{c0};
  while (!to_check.empty()) {
    auto c = to_check.front();
    to_check.pop_front();

    if (c.mana_spent >= min_mana) {
      continue;
    }
    if (c.player.hit_points <= 0) {
      continue;
    }
    if (c.boss.hit_points <= 0) {
      min_mana = std::min(c.mana_spent, min_mana);
      continue;
    }

    auto spells = ListAvailableSpells(c, all_spells);
    if (spells.empty()) {
      continue;
    }
    for (auto spell_idx : spells) {
      auto c2 = c;
      c2.spell_queue.push_back(all_spells[spell_idx]);

      SimulateFightTurn(&c2);
      to_check.push_back(c2);
    }
  }
  return min_mana;
}

int main() {
  std::vector<Spell> available_spells{
      {"Magic Missile", 53, 4, 0, std::nullopt},
      {"Drain", 73, 2, 2, std::nullopt},
      {"Shield", 113, 0, 0, Effect{EffectType::Shield, 6, 7}},
      {"Poison", 173, 0, 0, Effect{EffectType::Poison, 6, 3}},
      {"Recharge", 229, 0, 0, Effect{EffectType::Recharge, 5, 101}},
  };

  FightContext c;
  c.boss = {58, 9};
  c.player = {50, 0, 0, 500};

  auto min_mana = Simulate(c, available_spells);
  std::cout << min_mana << "\n";
  return 0;
}
