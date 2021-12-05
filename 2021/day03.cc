#include <algorithm>
#include <cassert>
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

std::vector<int> GetBitCount(std::vector<std::string> const &numbers) {
  std::vector<int> r(numbers.front().size(), 0);
  for (auto const &t : numbers) {
    NoteOnes(&r, t);
  }
  return r;
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

uint64_t BinaryInStringToNumber(std::string const &s) {
  uint64_t r = 0;
  for (auto b : s) {
    r <<= 1;
    r += ((b == '1') ? 1 : 0);
  }
  return r;
}

uint64_t FindOxygenGeneratorRating(std::vector<std::string> const &numbers,
                                   size_t idx) {
  auto const bit_count = GetBitCount(numbers);
  auto const most_common =
      ((bit_count[idx] >= (numbers.size() / 2 + numbers.size() % 2)) ? '1'
                                                                     : '0');
  std::vector<std::string> matching;
  std::remove_copy_if(
      numbers.begin(), numbers.end(), std::back_inserter(matching),
      [idx, most_common](auto t) { return t[idx] != most_common; });

  if (matching.size() > 1) {
    return FindOxygenGeneratorRating(
        matching, ((idx + 1 < numbers.front().size()) ? idx + 1 : 0));
  }
  assert(matching.size() == 1);
  return BinaryInStringToNumber(matching.front());
}

uint64_t FindCo2ScrubberRating(std::vector<std::string> const &numbers,
                               size_t idx) {
  auto const bit_count = GetBitCount(numbers);
  auto const least_common =
      ((bit_count[idx] < (numbers.size() / 2 + numbers.size() % 2)) ? '1'
                                                                    : '0');

  std::vector<std::string> matching;
  std::remove_copy_if(
      numbers.begin(), numbers.end(), std::back_inserter(matching),
      [idx, least_common](auto t) { return t[idx] != least_common; });
  if (matching.size() > 1) {
    return FindCo2ScrubberRating(
        matching, ((idx + 1 < numbers.front().size()) ? idx + 1 : 0));
  }
  assert(matching.size() == 1);
  return BinaryInStringToNumber(matching.front());
}

int main() {
  std::vector<std::string> numbers;

  std::string token;
  while (std::cin >> token) {
    numbers.push_back(token);
  }

  auto const bit_count = GetBitCount(numbers);
  auto const count = numbers.size();

  auto const gammaRate = GetMostCommon(bit_count, count);
  auto const epsilonRate = GetLeastCommon(bit_count, count);
  auto const powerConsumption = gammaRate * epsilonRate;
  std::cout << powerConsumption << "\n";

  auto const oxygenGeneratorRating = FindOxygenGeneratorRating(numbers, 0);
  auto const co2ScrubberRating = FindCo2ScrubberRating(numbers, 0);
  auto const lifeSupportRating = oxygenGeneratorRating * co2ScrubberRating;
  std::cout << lifeSupportRating << "\n";
  return 0;
}
