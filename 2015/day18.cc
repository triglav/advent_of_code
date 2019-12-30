#include <iostream>
#include <string>
#include <bitset>

int const kSize = 100;
using Lights = std::bitset<kSize * kSize>;

bool Check(Lights const &l, int x, int y) {
  if (x < 0 || x >= kSize) {
    return false;
  }
  if (y < 0 || y >= kSize) {
    return false;
  }
  return l[y * kSize + x];
}

int CountLitNeighbors(Lights const &l, int x, int y) {
  int c = 0;
  c += Check(l, x + 1, y + 1);
  c += Check(l, x + 1, y);
  c += Check(l, x + 1, y - 1);

  c += Check(l, x, y + 1);
  c += Check(l, x, y - 1);

  c += Check(l, x - 1, y + 1);
  c += Check(l, x - 1, y);
  c += Check(l, x - 1, y - 1);
  return c;
}

Lights Step(Lights const &l0) {
  Lights l1;
  int i = 0;
  for (int y = 0; y < kSize; ++y) {
    for (int x = 0; x < kSize; ++x, ++i) {
      auto const c = CountLitNeighbors(l0, x, y);
      if (l0[i]) {
        l1[i] = c == 2 || c == 3;
      } else {
        l1[i] = c == 3;
      }
    }
  }
  return l1;
}

int main() {
  Lights lights0;

  int i = 0;
  std::string line;
  while (std::getline(std::cin, line)) {
    for (auto c : line) {
      lights0[i++] = c == '#';
    }
  }

  auto lights = lights0;
  for (int i = 0; i < 100; ++i) {
    lights = Step(lights);
  }
  std::cout << lights.count() << "\n";
  return 0;
}
