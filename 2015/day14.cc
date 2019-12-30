#include <algorithm>
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

int Simulate(std::vector<Reindeer> const &reindeers, int total_duration) {
  std::vector<int> points(reindeers.size(), 0);

  for (int k = 1; k <= total_duration; ++k) {
    int max_distance = 0;
    std::vector<size_t> indices;

    for (int i = 0; i < reindeers.size(); ++i) {
      auto const d = CalculateDistanceAfterDuration(reindeers[i], k);
      if (d >= max_distance) {
        if (d > max_distance) {
          max_distance = d;
          indices.clear();
        }
        indices.push_back(i);
      }
    }
    for (auto i : indices) {
      points[i] += 1;
    }
  }
  auto it = std::max_element(points.begin(), points.end());
  return *it;
}

int main() {
  int const kTotalDuration = 2503;

  int max_distance = 0;

  std::vector<Reindeer> reindeers;

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

    reindeers.emplace_back(std::move(r));
  }
  std::cout << max_distance << "\n"
            << Simulate(reindeers, kTotalDuration) << "\n";

  return 0;
}
