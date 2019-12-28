#include <iostream>
#include <sstream>
#include <string>
#include <vector>

int main() {
  int paper = 0;

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

    if (l < w) {
      paper += l * std::min(w, h);
    } else {
      paper += w * std::min(l, h);
    }
  }
  std::cout << paper << "\n";
  return 0;
}
