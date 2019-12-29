#include <algorithm>
#include <iostream>
#include <string>
#include <unordered_map>

#include "string_utils.h"

using Destinations = std::unordered_map<std::string, int>;
using Graph = std::unordered_map<std::string, Destinations>;

void InsertPath(Graph *graph, std::string const &t0, std::string const &t1,
                int d) {
  auto const it = graph->find(t0);
  if (it != graph->end()) {
    it->second.emplace(t1, d);
    return;
  }
  Destinations ds;
  ds.emplace(t1, d);
  graph->emplace(t0, std::move(ds));
}

template <typename T> std::vector<T> Without(std::vector<T> v, T value) {
  auto it = std::remove(v.begin(), v.end(), value);
  v.erase(it, v.end());
  return v;
}

void SearchStep(Graph const &graph, std::vector<std::string> const &towns,
                std::string const &current_town, int distance,
                int *min_distance, int *max_distance) {
  if (towns.empty()) {
    *min_distance = std::min(*min_distance, distance);
    *max_distance = std::max(*max_distance, distance);
    return;
  }

  auto const &ds = graph.at(current_town);
  for (auto const &t : towns) {
    assert(t != current_town);
    auto towns2 = Without(towns, t);
    SearchStep(graph, towns2, t, distance + ds.at(t), min_distance,
               max_distance);
  }
}

std::pair<int, int> Search(Graph const &graph) {
  std::vector<std::string> all_towns;
  for (auto const &t : graph) {
    all_towns.push_back(t.first);
  }

  int min_distance = std::numeric_limits<int>::max();
  int max_distance = 0;
  for (auto const &t : all_towns) {
    auto towns2 = Without(all_towns, t);
    SearchStep(graph, towns2, t, 0, &min_distance, &max_distance);
  }
  return {min_distance, max_distance};
}

int main() {
  Graph graph;

  std::string line;
  while (std::getline(std::cin, line)) {
    auto tokens = SplitString(line);

    auto t0 = std::string(tokens[0]);
    auto t1 = std::string(tokens[2]);
    auto const d = sv2number<int>(tokens[4]);

    InsertPath(&graph, t0, t1, d);
    InsertPath(&graph, t1, t0, d);
  }
  auto distance = Search(graph);
  std::cout << distance.first << "\n" << distance.second << "\n";
  return 0;
}
