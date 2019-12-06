#include <algorithm>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

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

std::vector<std::string> ListOrbits(Graph const &graph, std::string const &object) {
  std::vector<std::string> list;
  std::string o = object;
  while(o != "COM") {
    o = graph.find(o)->second;
    list.push_back(o);
  }
  std::reverse(list.begin(), list.end());
  return list;
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
  {
    int count = 0;
    for (auto const &entry : graph) {
      count += CountOrbits(graph, entry.first);
    }
    std::cout << count << "\n";
  }

  auto you = ListOrbits(graph, "YOU");
  auto san = ListOrbits(graph, "SAN");

  int x = 0;
  for (size_t i = 0; i < you.size() && i < san.size(); ++i) {
    if (you[i] != san[i]) {
      break;
    }
    ++x;
  }
  auto result = you.size() + san.size() - 2*x;
  std::cout << result << "\n";

  return 0;
}

