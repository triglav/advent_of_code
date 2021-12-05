#include <iostream>
#include <string>
#include <vector>

void NoteOnes(std::vector<int> *bit_count, std::string token) {
  for (size_t i = 0; i < token.size(); ++i) {
    if (token[i] == '1') {
      (*bit_count)[i] += 1;
    }
  }
}

uint64_t GetMostCommon(std::vector<int> const &bit_count, int count) {
  auto const x = count / 2;
  uint64_t r = 0;
  for (auto b : bit_count) {
    r <<= 1;
    r += ((b >= x) ? 1 : 0);
  }
  return r;
}

uint64_t GetLeastCommon(std::vector<int> const &bit_count, int count) {
  auto const x = count / 2;
  uint64_t r = 0;
  for (auto b : bit_count) {
    r <<= 1;
    r += ((b < x) ? 1 : 0);
  }
  return r;
}

int main() {
  std::string token;
  std::cin >> token;

  std::vector<int> bit_count(token.size(), 0);
  NoteOnes(&bit_count, token);
  int count = 1;
  while (std::cin >> token) {
    NoteOnes(&bit_count, token);
    ++count;
  }
  auto const gammaRate = GetMostCommon(bit_count, count);
  auto const epsilonRate = GetLeastCommon(bit_count, count);
  auto const powerConsumption = gammaRate * epsilonRate;
  std::cout << powerConsumption << "\n";
  return 0;
}
