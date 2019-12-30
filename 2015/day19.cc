#include <algorithm>
#include <iostream>
#include <set>
#include <string>

#include "string_utils.h"

std::set<std::string> Replace(std::string const &molecule,
                              std::string const &pattern,
                              std::string const &replacement) {
  std::set<std::string> r;
  size_t p0 = 0;
  for (auto p = molecule.find(pattern, p0); p != std::string::npos;
       p = molecule.find(pattern, p0)) {
    auto m2 = molecule;
    m2.replace(p, pattern.size(), replacement);
    r.insert(m2);
    p0 = p + 1;
  }
  return r;
}

int CountElements(std::string_view s, std::string_view subs) {
  int count = 0;
  auto p = s.find(subs, 0);
  while (p != std::string::npos) {
    ++count;
    p = s.find(subs, p + subs.length());
  }
  return count;
}

int main() {
  std::vector<std::pair<std::string, std::string>> replacements;
  std::string line;
  while (std::getline(std::cin, line)) {
    if (line.empty()) {
      break;
    }

    auto t = SplitString(line);
    replacements.emplace_back(std::string(t[0]), std::string(t[2]));
  }

  std::string molecule;
  std::getline(std::cin, molecule);

  std::set<std::string> molecules;
  for (auto const &[p, r] : replacements) {
    auto new_molecules = Replace(molecule, p, r);
    molecules.merge(new_molecules);
  }
  std::cout << molecules.size() << "\n";

  auto const count_all =
      std::count_if(molecule.begin(), molecule.end(),
                    [](char c) { return c >= 'A' && c <= 'Z'; });

  auto const count_Rn = CountElements(molecule, "Rn");
  auto const count_Ar = CountElements(molecule, "Ar");
  auto const count_Y = CountElements(molecule, "Y");
  auto const result = count_all - count_Rn - count_Ar - 2 * count_Y - 1;
  std::cout << result << "\n";
  return 0;
}
