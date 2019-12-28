#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

using Deck = std::vector<__int128>;

Deck FactoryDeck(__int128 c) {
  Deck deck;
  deck.reserve(c);
  for (__int128 i = 0; i < c; ++i) {
    deck.push_back(i);
  }
  return deck;
}

enum class Technique : char {
  Cut,
  Increment,
  NewStack,
};

using Techniques = std::vector<std::pair<Technique, __int128>>;

Techniques ReadTechniques() {
  Techniques techniques;
  for (std::string line; std::getline(std::cin, line);) {
    if (line.rfind("cut", 0) == 0) {
      auto const token = line.substr(4);
      __int128 const x = std::stoll(token);
      techniques.emplace_back(Technique::Cut, x);
    } else if (line.rfind("deal with increment", 0) == 0) {
      auto const token = line.substr(20);
      __int128 const step = std::stoll(token);
      techniques.emplace_back(Technique::Increment, step);
    } else if (line.rfind("deal into new stack", 0) == 0) {
      techniques.emplace_back(Technique::NewStack, -1);
    }
  }
  return techniques;
}

Deck Shuffle(Deck deck, Techniques const &techniques) {
  for (auto const &t : techniques) {
    if (t.first == Technique::Cut) {
      auto const x = t.second;
      if (x > 0) {
        std::rotate(deck.begin(), deck.begin() + x, deck.end());
      } else {
        std::rotate(deck.rbegin(), deck.rbegin() - x, deck.rend());
      }
    } else if (t.first == Technique::Increment) {
      auto const step = t.second;
      auto const size = deck.size();

      auto deck2 = std::move(deck);
      deck = Deck(size, -1);

      __int128 i = 0;
      for (auto it = deck2.begin(); it != deck2.end(); ++it) {
        deck[i] = *it;
        i = (i + step) % size;
      }
    } else if (t.first == Technique::NewStack) {
      std::reverse(deck.begin(), deck.end());
    } else {
      std::cerr << "err\n";
      exit(1);
    }
  }
  return deck;
}

__int128 pow(__int128 base, __int128 exp, __int128 c) {
  if (exp == 0) {
    return 1;
  }
  __int128 a = pow(base, exp / 2, c);
  a = a * a % c;
  if (exp & 1) {
    return (a * base) % c;
  }
  return a;
}

int main() {
  auto techniques = ReadTechniques();
  {
    auto deck = FactoryDeck(10007);
    deck = Shuffle(deck, techniques);
    auto const it = std::find(deck.begin(), deck.end(), 2019);
    auto const pos = it - deck.begin();
    std::cout << pos << "\n";
  }
  {
    __int128 const kDeckSize = 119315717514047;
    __int128 const kShuffleCount = 101741582076661;

    __int128 a = 1;
    __int128 b = 0;
    for (auto it = techniques.rbegin(); it != techniques.rend(); ++it) {
      auto const &t = *it;
      if (t.first == Technique::Increment) {
        auto const inc = t.second;
        __int128 const p = pow(inc, kDeckSize - 2, kDeckSize);
        a *= p;
        b *= p;
      } else if (t.first == Technique::Cut) {
        auto const inc = t.second;
        b += inc;
      } else if (t.first == Technique::NewStack) {
        b += 1;
        a *= -1;
        b *= -1;
      } else {
        std::cerr << "err\n";
        exit(1);
      }
      a %= kDeckSize;
      while (a < 0) {
        a += kDeckSize;
      }
      b %= kDeckSize;
      while (b < 0) {
        b += kDeckSize;
      }
    }

    auto x1 = pow(a, kShuffleCount, kDeckSize);
    auto x2 = x1 + kDeckSize - 1;
    auto x3 = pow(a - 1, kDeckSize - 2, kDeckSize);
    auto r = (x1 * 2020 + ((b * x2) % kDeckSize) * x3) % kDeckSize;
    std::cout << static_cast<uint64_t>(r) << "\n";
  }
  return 0;
}
