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
  return 0;
}
