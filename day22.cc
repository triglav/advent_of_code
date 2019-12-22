#include <algorithm>
#include <iostream>
#include <string>
#include <string_view>
#include <vector>

using Deck = std::vector<int64_t>;

Deck FactoryDeck(size_t c) {
  Deck deck;
  for (int64_t i = 0; i < c; ++i) {
    deck.push_back(i);
  }
  return deck;
}

int main() {
  auto deck = FactoryDeck(10007);

  for (std::string line; std::getline(std::cin, line);) {
    if (line.rfind("cut", 0) == 0) {
      auto const token = line.substr(4);
      int64_t const x = std::stoll(token);
      if (x > 0) {
        std::rotate(deck.begin(), deck.begin() + x, deck.end());
      } else {
        std::rotate(deck.rbegin(), deck.rbegin() - x, deck.rend());
      }
    } else if (line.rfind("deal with increment", 0) == 0) {
      auto const token = line.substr(20);
      int64_t const step = std::stoll(token);
      auto const size = deck.size();

      auto deck2 = std::move(deck);
      deck = Deck(size, -1);

      int64_t i = 0;
      for (auto it = deck2.begin(); it != deck2.end(); ++it) {
        deck[i] = *it;
        i = (i + step) % size;
      }
    } else if (line.rfind("deal into new stack", 0) == 0) {
      std::reverse(deck.begin(), deck.end());
    }
  }
  auto const it = std::find(deck.begin(), deck.end(), 2019);
  auto const pos = it - deck.begin();
  std::cout << pos << "\n";
  return 0;
}

