#include <cassert>
#include <iostream>
#include <string>
#include <string_view>
#include <unordered_map>
#include <variant>
#include <vector>

#include "string_utils.h"

std::vector<std::string_view> SplitString(std::string_view s, char delimiter) {
  std::vector<std::string_view> r;
  size_t p0 = 0;
  size_t p1 = s.find(delimiter);

  while (p1 != std::string::npos) {
    r.push_back(s.substr(p0, p1 - p0));

    p0 = p1 + 1;
    p1 = s.find(delimiter, p0);
  }

  r.push_back(s.substr(p0));
  return r;
}

enum class CommandType {
  Value = 0,
  Not,
  And,
  Or,
  RShift,
  LShift,
};

using Value = uint16_t;
using CommandParam = std::variant<std::string, Value>;

struct Command {
  CommandType type{CommandType::Value};
  std::vector<CommandParam> params;
};

CommandType ParseCommandType(std::string_view type) {
  if (type == "NOT") {
    return CommandType::Not;
  }
  if (type == "AND") {
    return CommandType::And;
  }
  if (type == "OR") {
    return CommandType::Or;
  }
  if (type == "RSHIFT") {
    return CommandType::RShift;
  }
  if (type == "LSHIFT") {
    return CommandType::LShift;
  }
  std::cerr << "Unknown command: " << type << "\n";
  exit(1);
}

std::pair<std::string, Command>
ParseCommand(std::vector<std::string_view> const &tokens) {
  Command command;
  for (auto t : tokens) {
    auto c = t[0];
    if (c >= 'a' && c <= 'z') {
      command.params.push_back(std::string(t));
      continue;
    }
    if (c >= 'A' && c <= 'Z') {
      command.type = ParseCommandType(t);
      continue;
    }
    if (c >= '0' && c <= '9') {
      auto n = sv2number<Value>(t);
      command.params.push_back(n);
      continue;
    }
    if (t == "->") {
      continue;
    }
    assert(false);
  }
  auto result = std::get<std::string>(command.params.back());
  command.params.pop_back();
  return {result, command};
}

using Circuit = std::unordered_map<std::string, Command>;
using Buffer = std::unordered_map<std::string, Value>;

Value Evaluate(Circuit const &circuit, std::string const &wire, Buffer *buffer);

Value GetParamValue(Circuit const &circuit, CommandParam const &p,
                    Buffer *buffer) {
  if (std::holds_alternative<std::string>(p)) {
    auto const wire2 = std::get<std::string>(p);

    auto const it = buffer->find(wire2);
    if (it != buffer->end()) {
      return it->second;
    }
    auto const v = Evaluate(circuit, wire2, buffer);
    buffer->insert({wire2, v});
    return v;
  }
  assert(std::holds_alternative<Value>(p));
  return std::get<Value>(p);
}

Value Evaluate(Circuit const &circuit, std::string const &wire,
               Buffer *buffer) {
  auto c = circuit.at(wire);
  switch (c.type) {
  case CommandType::Value: {
    assert(c.params.size() == 1);
    auto const &p = c.params.front();
    auto const v = GetParamValue(circuit, p, buffer);
    return v;
  }
  case CommandType::Not: {
    assert(c.params.size() == 1);
    auto const &p = c.params.front();
    auto const v = GetParamValue(circuit, p, buffer);
    return ~v;
  }
  case CommandType::And: {
    assert(c.params.size() == 2);
    auto const &p0 = c.params[0];
    auto const &p1 = c.params[1];
    auto const v0 = GetParamValue(circuit, p0, buffer);
    auto const v1 = GetParamValue(circuit, p1, buffer);
    return v0 & v1;
  }
  case CommandType::Or: {
    assert(c.params.size() == 2);
    auto const &p0 = c.params[0];
    auto const &p1 = c.params[1];
    auto const v0 = GetParamValue(circuit, p0, buffer);
    auto const v1 = GetParamValue(circuit, p1, buffer);
    return v0 | v1;
  }
  case CommandType::RShift: {
    assert(c.params.size() == 2);
    auto const &p0 = c.params[0];
    auto const &p1 = c.params[1];
    auto const v0 = GetParamValue(circuit, p0, buffer);
    auto const v1 = GetParamValue(circuit, p1, buffer);
    return v0 >> v1;
  }
  case CommandType::LShift: {
    assert(c.params.size() == 2);
    auto const &p0 = c.params[0];
    auto const &p1 = c.params[1];
    auto const v0 = GetParamValue(circuit, p0, buffer);
    auto const v1 = GetParamValue(circuit, p1, buffer);
    return v0 << v1;
  }
  default:
    std::cerr << "Unknown command type (" << wire << ")\n";
    exit(2);
  }
  return 0;
}

int main() {
  Circuit circuit;

  std::string line;
  while (std::getline(std::cin, line)) {
    auto const v = SplitString(line, ' ');
    auto [r, c] = ParseCommand(v);
    circuit.emplace(r, c);
  }
  Buffer value_buffer;
  auto r = Evaluate(circuit, "a", &value_buffer);
  std::cout << r << "\n";
}
