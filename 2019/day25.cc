#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdint>
#include <deque>
#include <fstream>
#include <iostream>
#include <optional>
#include <string>
#include <tuple>
#include <vector>

using Memory = std::vector<int64_t>;

enum class Signal {
  END = 0,
  ERR,
  INT,
};

class Program {
public:
  explicit Program(Memory const &memory)
      : memory_(memory), pos_(0), relative_base_(0), state_(Signal::INT) {}

  std::tuple<Signal, int64_t> Execute(std::deque<int64_t> *inputs) {
    if (state_ != Signal::INT) {
      return std::make_tuple(state_, 0);
    }
    while (true) {
      auto const instruction = ReadMemory(pos_++);
      auto const opcode = instruction % 100;
      switch (opcode) {
      case 99:
        state_ = Signal::END;
        return std::make_tuple(Signal::END, 0);
      case 1: {
        auto const p1 = GetParamValue(1, instruction);
        auto const p2 = GetParamValue(2, instruction);
        auto const output_pos = GetParamAddress(3, instruction);
        WriteMemory(output_pos, p1 + p2);
        break;
      }
      case 2: {
        auto const p1 = GetParamValue(1, instruction);
        auto const p2 = GetParamValue(2, instruction);
        auto const output_pos = GetParamAddress(3, instruction);
        WriteMemory(output_pos, p1 * p2);
        break;
      }
      case 3: {
        auto const output_pos = GetParamAddress(1, instruction);
        if (inputs->empty()) {
          state_ = Signal::INT;
          return std::make_tuple(Signal::INT, -1);
        }
        WriteMemory(output_pos, inputs->front());
        inputs->pop_front();
        break;
      }
      case 4: {
        auto const x = GetParamValue(1, instruction);
        state_ = Signal::INT;
        return std::make_tuple(Signal::INT, x);
      }
      case 5: {
        auto const p1 = GetParamValue(1, instruction);
        auto const p2 = GetParamValue(2, instruction);
        if (p1 != 0) {
          pos_ = p2;
        }
        break;
      }
      case 6: {
        auto const p1 = GetParamValue(1, instruction);
        auto const p2 = GetParamValue(2, instruction);
        if (p1 == 0) {
          pos_ = p2;
        }
        break;
      }
      case 7: {
        auto const p1 = GetParamValue(1, instruction);
        auto const p2 = GetParamValue(2, instruction);
        auto const output_pos = GetParamAddress(3, instruction);
        WriteMemory(output_pos, ((p1 < p2) ? 1 : 0));
        break;
      }
      case 8: {
        auto const p1 = GetParamValue(1, instruction);
        auto const p2 = GetParamValue(2, instruction);
        auto const output_pos = GetParamAddress(3, instruction);
        WriteMemory(output_pos, ((p1 == p2) ? 1 : 0));
        break;
      }
      case 9: {
        auto const p = GetParamValue(1, instruction);
        relative_base_ += p;
        break;
      }
      default:
        std::cerr << "Invalid opcode: " << opcode << "\n";
        state_ = Signal::ERR;
        return std::make_tuple(Signal::ERR, 0);
      }
    }
    std::cerr << "Unexpected error\n";
    state_ = Signal::ERR;
    return std::make_tuple(Signal::ERR, 0);
  }

private:
  int64_t GetParamAddress(int64_t number, int64_t instruction) {
    auto const mode =
        (instruction / static_cast<int64_t>(std::pow(10, number + 1))) % 10;
    if (mode == 0) {
      auto const input_pos = ReadMemory(pos_++);
      return input_pos;
    }
    if (mode == 2) {
      auto const offset = ReadMemory(pos_++);
      return relative_base_ + offset;
    }
    assert(mode == 1);
    return pos_++;
  }

  int64_t GetParamValue(int64_t number, int64_t instruction) {
    auto const address = GetParamAddress(number, instruction);
    return ReadMemory(address);
  }

  void WriteMemory(int64_t p, int64_t value) {
    if (memory_.size() <= p) {
      memory_.resize(p + 1, 0);
    }
    memory_[p] = value;
  }

  int64_t ReadMemory(int64_t p) {
    if (memory_.size() <= p) {
      memory_.resize(p + 1, 0);
    }
    return memory_[p];
  }

private:
  Memory memory_;
  int64_t pos_;
  int64_t relative_base_;
  Signal state_;
}; // class Program

std::string AdjustCommand(std::string const &command) {
  if (command == "n") {
    return "north";
  }
  if (command == "w") {
    return "west";
  }
  if (command == "s") {
    return "south";
  }
  if (command == "e") {
    return "east";
  }
  return command;
}

void AddCommand(std::deque<int64_t> *inputs, std::string const &command) {
  std::cout << "-> " << command;
  for (auto c : command) {
    inputs->push_back(c);
  }
  if (command.back() != '\n') {
    std::cout << "\n";
    inputs->push_back('\n');
  }
}

std::string ReadLine(Program *program, std::deque<int64_t> *inputs) {
  std::string output;
  while (true) {
    auto const [s, v] = program->Execute(inputs);
    if (s == Signal::END) {
      std::cout << "end\n";
      exit(0);
    }
    if (s == Signal::ERR) {
      std::cerr << "oops\n";
      exit(1);
    }
    auto c = static_cast<char>(v);
    output.push_back(c);
    if (c == '\n') {
      break;
    }
  }
  std::cout << output;
  return output;
}

using Door = std::string;

Door OppositeDoor(Door const &d) {
  if (d == "north") {
    return "south";
  }
  if (d == "east") {
    return "west";
  }
  if (d == "south") {
    return "north";
  }
  if (d == "west") {
    return "east";
  }
  assert(false);
}

struct Room {
  std::string name;
  std::vector<std::string> items;
  std::unordered_map<Door, std::optional<std::string>> doors;
};

Room ReadCurrentRoom(Program *program, std::deque<int64_t> *inputs) {
  Room r;
  auto line = ReadLine(program, inputs);
  while (true) {
    if (line.rfind("==", 0) == 0) {
      auto place = line.substr(3, line.size() - 7);
      r.name = place;
    } else if (line == "Doors here lead:\n") {
      for (line = ReadLine(program, inputs); line.rfind("- ", 0) == 0;
           line = ReadLine(program, inputs)) {
        auto door = line.substr(2, line.size() - 3);
        r.doors[door] = std::nullopt;
      }
      continue;
    } else if (line == "Items here:\n") {
      for (line = ReadLine(program, inputs); line.rfind("- ", 0) == 0;
           line = ReadLine(program, inputs)) {
        auto item = line.substr(2, line.size() - 3);
        r.items.push_back(item);
      }
      continue;
    }
    if (line == "Command?\n") {
      break;
    }
    line = ReadLine(program, inputs);
  }
  return r;
}

std::string Explore(Program *program, std::deque<int64_t> *inputs,
                    std::unordered_map<std::string, Room> *rooms,
                    Door const &last_door) {
  auto r = ReadCurrentRoom(program, inputs);
  assert(!r.name.empty());

  if (rooms->find(r.name) != rooms->end()) {
    return r.name;
  }

  for (auto const &i : r.items) {
    if (i == "escape pod" || i == "giant electromagnet" || i == "photons" ||
        i == "infinite loop" || i == "molten lava") {
      continue;
    }
    auto command = "take " + i;
    AddCommand(inputs, command);
    auto rx = ReadCurrentRoom(program, inputs);
    assert(r.name == rx.name || rx.name.empty());
  }

  rooms->emplace(r.name, r);

  for (auto const &[d, room_name] : r.doors) {
    if (d == last_door) {
      continue;
    }
    AddCommand(inputs, d);

    auto d2 = OppositeDoor(d);
    auto r2_name = Explore(program, inputs, rooms, d2);
    if (r2_name != r.name) {
      rooms->at(r.name).doors[d] = r2_name;
      rooms->at(r2_name).doors[d2] = r.name;

      std::cout << "going back: ";

      AddCommand(inputs, d2);
      ReadCurrentRoom(program, inputs);
      assert(!r.name.empty());
    }
  }
  return r.name;
}

std::optional<std::vector<std::string>>
FindPath(Program *program, std::deque<int64_t> *inputs,
         std::unordered_map<std::string, Room> const &rooms,
         std::string current_location, std::string const &destination,
         std::vector<std::string> directions) {
  if (current_location == destination) {
    return directions;
  }
  std::optional<std::vector<std::string>> r;
  for (auto const &[door, l] : rooms.at(current_location).doors) {
    if (!directions.empty() && directions.back() == OppositeDoor(door)) {
      continue;
    }
    assert(l.has_value());
    auto d = directions;
    d.push_back(door);
    auto r2 = FindPath(program, inputs, rooms, *l, destination, d);
    if (r2.has_value() && (!r.has_value() || r2->size() < r2->size())) {
      r = r2;
    }
  }
  return r;
}

void GoTo(Program *program, std::deque<int64_t> *inputs,
          std::unordered_map<std::string, Room> const &rooms,
          std::string current_location, std::string const &destination) {
  auto path =
      FindPath(program, inputs, rooms, current_location, destination, {});
  assert(path.has_value());
  Room r;
  for (auto const &d : *path) {
    AddCommand(inputs, d);
    r = ReadCurrentRoom(program, inputs);
  }
  assert(r.name == destination);
}

std::vector<std::string> CheckInventory(Program *program,
                                        std::deque<int64_t> *inputs) {
  AddCommand(inputs, "inv");
  std::vector<std::string> items;
  auto line = ReadLine(program, inputs);
  while (true) {
    if (line == "Items in your inventory:\n") {
      for (line = ReadLine(program, inputs); line.rfind("- ", 0) == 0;
           line = ReadLine(program, inputs)) {
        auto item = line.substr(2, line.size() - 3);
        items.push_back(item);
      }
      continue;
    }
    if (line == "Command?\n") {
      break;
    }
    line = ReadLine(program, inputs);
  }
  return items;
}

template <typename T>
void CombinationStep(std::vector<T> const &all_items, int start,
                     int remaining_count, std::vector<T> const &current,
                     std::vector<std::vector<T>> *combinations) {
  if (remaining_count <= 0) {
    combinations->push_back(current);
    return;
  }

  for (int i = start; i < all_items.size() - remaining_count + 1; ++i) {
    auto v = current;
    v.push_back(all_items[i]);
    CombinationStep(all_items, i + 1, remaining_count - 1, v, combinations);
  }
}

template <typename T>
std::vector<std::vector<T>> GetCombinations(std::vector<T> const &all_items) {
  std::vector<std::vector<T>> combinations;
  for (int l = 1; l <= all_items.size(); ++l) {
    CombinationStep(all_items, 0, l, {}, &combinations);
  }
  return combinations;
}

void DropItems(Program *program, std::deque<int64_t> *inputs,
               std::vector<std::string> const &items) {
  for (auto const &i : items) {
    AddCommand(inputs, "drop " + i);
    ReadCurrentRoom(program, inputs);
  }
}

void TakeItems(Program *program, std::deque<int64_t> *inputs,
               std::vector<std::string> const &items) {
  for (auto const &i : items) {
    AddCommand(inputs, "take " + i);
    ReadCurrentRoom(program, inputs);
  }
}

void PassPressureSensitiveFloor(
    Program *program, std::deque<int64_t> *inputs,
    std::unordered_map<std::string, Room> const &rooms,
    std::string current_location, std::string const &direction) {

  assert(rooms.at(current_location).doors.find(direction) !=
         rooms.at(current_location).doors.end());

  auto const all_items = CheckInventory(program, inputs);

  DropItems(program, inputs, all_items);

  auto combinations = GetCombinations(all_items);
  for (auto const &c : combinations) {
    TakeItems(program, inputs, c);

    AddCommand(inputs, direction);
    auto r = ReadCurrentRoom(program, inputs);
    if (r.name != current_location) {
      std::cout << "Reached '" << r.name << "'\n";
      break;
    }
    DropItems(program, inputs, c);
  }
}

int main() {
  Memory memory;
  {
    std::string token;
    std::ifstream is("input25.txt");
    while (std::getline(is, token, ',')) {
      auto instruction = std::stol(token);
      memory.push_back(instruction);
    }
  }

  std::unordered_map<std::string, Room> rooms;
  {
    Program program{memory};
    std::deque<int64_t> inputs;

#if 0
    ReadCurrentRoom(&program, &inputs);
#else
    auto r_name = Explore(&program, &inputs, &rooms, "");
    GoTo(&program, &inputs, rooms, r_name, "Security Checkpoint");
    PassPressureSensitiveFloor(&program, &inputs, rooms, "Security Checkpoint",
                               "west");
#endif

    while (true) {
      std::string command;
      std::getline(std::cin, command);
      command = AdjustCommand(command);

      AddCommand(&inputs, command);
      auto r = ReadCurrentRoom(&program, &inputs);
    }
  }
  return 0;
}
