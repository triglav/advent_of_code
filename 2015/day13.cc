#include <iostream>
#include <string>
#include <unordered_map>

#include "set_utils.h"
#include "string_utils.h"

using Happiness = std::unordered_map<std::string, int>;
using Attendees = std::unordered_map<std::string, Happiness>;

void InsertData(Attendees *attendees, std::string const &n1,
                std::string const &n2, int happiness) {
  auto it = attendees->find(n1);
  if (it == attendees->end()) {
    auto p = attendees->emplace(n1, Happiness());
    it = p.first;
  }
  it->second.emplace(n2, happiness);
}

int CalculateHappiness(Attendees const &attendees,
                       std::vector<std::string> const &variation) {
  int r = 0;
  for (int i = 0; i < variation.size(); ++i) {
    auto const &h = attendees.at(variation[i]);

    auto const &prev = variation[(i - 1 + variation.size()) % variation.size()];
    auto const &next = variation[(i + 1) % variation.size()];

    r += h.at(prev);
    r += h.at(next);
  }
  return r;
}

int main() {
  Attendees attendees;

  std::string line;
  while (std::getline(std::cin, line)) {
    auto t = SplitString(line);

    auto const n1 = std::string(t[0]);
    auto const n2 = std::string(t[10].substr(0, t[10].size() - 1));

    assert(t[2] == "gain" || t[2] == "lose");
    auto const sign = ((t[2] == "gain") ? 1 : -1);
    auto const v = sv2number<int>(t[3]) * sign;

    InsertData(&attendees, n1, n2, v);
  }
  auto v = GetVariationsRound(Keys(attendees));

  int max_happiness = std::numeric_limits<int>::min();
  for (auto const &v2 : v) {
    auto const h = CalculateHappiness(attendees, v2);
    max_happiness = std::max(max_happiness, h);
  }
  std::cout << max_happiness << "\n";
  return 0;
}
