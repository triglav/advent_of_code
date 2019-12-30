#include <iostream>
#include <string>

#include "string_utils.h"

struct Reindeer {
  int flight_speed;
  int flight_duration;
  int rest_duration;
};

int CalculateDistanceAfterDuration(Reindeer const &r, int total_duration) {
  auto const t = r.flight_duration + r.rest_duration;

  auto const x = total_duration / t;
  auto const y = total_duration % t;

  return x * r.flight_duration * r.flight_speed +
         std::min(y, r.flight_duration) * r.flight_speed;
}

int main() {
  int const kTotalDuration = 2503;

  int max_distance = 0;

  std::string line;
  while (std::getline(std::cin, line)) {
    auto const t = SplitString(line);

    auto r = Reindeer{
        sv2number<int>(t[3]),
        sv2number<int>(t[6]),
        sv2number<int>(t[13]),
    };

    auto const distance = CalculateDistanceAfterDuration(r, kTotalDuration);
    max_distance = std::max(distance, max_distance);
  }
  std::cout << max_distance << "\n";
  return 0;
}
