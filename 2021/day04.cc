#include <array>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

class Board {
  std::array<int, 25> numbers_;
  std::array<bool, 25> marks_{false};

public:
  friend std::istream &operator>>(std::istream &input, Board &board) {
    int n;
    for (int i = 0; i < 25; ++i) {
      input >> n;
      board.numbers_[i] = n;
    }
    return input;
  }

  void Mark(int n) {
    for (size_t i = 0; i < numbers_.size(); ++i) {
      if (numbers_[i] == n) {
        marks_[i] = true;
        break;
      }
    }
  }

  bool winning() const {
    return is_row_complete(0) || is_row_complete(1) || is_row_complete(2) ||
           is_row_complete(3) || is_row_complete(4) || is_column_complete(0) ||
           is_column_complete(1) || is_column_complete(2) ||
           is_column_complete(3) || is_column_complete(4);
  }

  int CalculateScore() const {
    int sum = 0;
    for (int i = 0; i < 25; ++i) {
      if (!marks_[i]) {
        sum += numbers_[i];
      }
    }
    return sum;
  }

private:
  bool is_row_complete(int i) const {
    const auto idx = i * 5;
    return marks_[idx] && marks_[idx + 1] && marks_[idx + 2] &&
           marks_[idx + 3] && marks_[idx + 4];
  }

  bool is_column_complete(int i) const {
    return marks_[i + 5 * 0] && marks_[i + 5 * 1] && marks_[i + 5 * 2] &&
           marks_[i + 5 * 3] && marks_[i + 5 * 4];
  }
};

int main() {
  std::string draw_numbers_input;
  std::getline(std::cin, draw_numbers_input);

  std::vector<Board> boards;
  Board b;
  while (std::cin >> b) {
    boards.push_back(b);
  }

  std::stringstream ss(draw_numbers_input);
  std::string token;
  while (std::getline(ss, token, ',')) {
    auto const n = std::stoi(token);
    for (auto it = boards.begin(); it != boards.end();) {
      it->Mark(n);
      if (!it->winning()) {
        ++it;
        continue;
      }
      if (boards.size() > 1) {
        it = boards.erase(it);
        continue;
      }
      auto const final_score = it->CalculateScore() * n;
      std::cout << final_score << "\n";
      return 0;
    }
  }
  return 1;
}
