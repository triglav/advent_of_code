#include <iostream>
#include <sstream>
#include <unordered_map>
#include <vector>

#include "coord.h"

struct VentLine {
  Coord p1;
  Coord p2;
};

std::istream &operator>>(std::istream &input, VentLine &v) {
  std::string token;

  std::getline(input, token, ',');
  v.p1.x = std::stoi(token);

  std::getline(input, token, ' ');
  v.p1.y = std::stoi(token);

  std::getline(input, token, ' ');

  std::getline(input, token, ',');
  v.p2.x = std::stoi(token);

  std::getline(input, token, '\n');
  v.p2.y = std::stoi(token);

  return input;
}

void AddVent(std::unordered_map<Coord, int> *vents, Coord p) {
  auto const it = vents->find(p);
  auto const c = ((it != vents->end()) ? it->second : 0);
  vents->insert_or_assign(p, c + 1);
}

int main() {
  std::unordered_map<Coord, int> vents;

  std::string line;
  while (std::getline(std::cin, line)) {
    std::stringstream ss(line);
    VentLine v;
    ss >> v;
    if (v.p1.x == v.p2.x) {
      for (int y = std::min(v.p1.y, v.p2.y); y <= std::max(v.p1.y, v.p2.y);
           ++y) {
        AddVent(&vents, Coord{v.p1.x, y});
      }
    } else if (v.p1.y == v.p2.y) {
      for (int x = std::min(v.p1.x, v.p2.x); x <= std::max(v.p1.x, v.p2.x);
           ++x) {
        AddVent(&vents, Coord{x, v.p1.y});
      }
    }
  }

  int count = 0;
  for (auto const &p : vents) {
    if (p.second >= 2) {
      ++count;
    }
  }
  std::cout << count << "\n";

  return 0;
}
