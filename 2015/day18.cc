#include <bitset>
#include <iostream>
#include <string>

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

void AdjustCorners(Lights *l) {
  l->set(0);
  l->set(kSize - 1);
  l->set((kSize - 1) * kSize);
  l->set((kSize - 1) * kSize + kSize - 1);
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
  auto lights2 = lights0;
  AdjustCorners(&lights2);
  for (int i = 0; i < 100; ++i) {
    lights = Step(lights);

    lights2 = Step(lights2);
    AdjustCorners(&lights2);
  }
  std::cout << lights.count() << "\n" << lights2.count() << "\n";
  return 0;
}
