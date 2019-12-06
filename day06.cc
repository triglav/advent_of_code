#include <iostream>
#include <string>
#include <unordered_map>

using Graph = std::unordered_map<std::string, std::string>;

int CountOrbits(Graph const &graph, std::string const &object) {
  int count = 0;
  std::string o = object;
  while(o != "COM") {
    o = graph.find(o)->second;
    ++count;
  }
  return count;
}

int main() {
  Graph graph;

  std::string token;
  while (std::getline(std::cin, token, ')')) {
    auto const p1 = token;
    std::getline(std::cin, token);
    auto const p2 = token;

    graph.emplace(p2, p1);
  }
  int count = 0;
  for (auto const &entry : graph) {
    count += CountOrbits(graph, entry.first);
  }
  std::cout << count << "\n";
  return 0;
}

