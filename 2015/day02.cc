#include <iostream>
#include <sstream>
#include <string>
#include <vector>

int main() {
  int paper = 0;
  int ribbon = 0;

  std::string line;
  while (std::getline(std::cin, line)) {
    std::stringstream ss(line);
    std::string token;

    std::getline(ss, token, 'x');
    auto const l = std::stoi(token);
    std::getline(ss, token, 'x');
    auto const w = std::stoi(token);
    std::getline(ss, token, '\n');
    auto const h = std::stoi(token);

    paper += 2 * l * w + 2 * l * h + 2 * w * h;
    ribbon += l * w * h;

    if (l < w) {
      auto const x = std::min(w, h);
      paper += l * x;
      ribbon += 2 * l + 2 * x;
    } else {
      auto const x = std::min(l, h);
      paper += w * x;
      ribbon += 2 * w + 2 * x;
    }
  }
  std::cout << paper << "\n" << ribbon << "\n";
  return 0;
}
