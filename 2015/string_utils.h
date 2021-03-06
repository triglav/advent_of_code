#pragma once

#include <cassert>
#include <charconv>
#include <string_view>
#include <vector>

template <typename T> T sv2number(std::string_view s) {
  T number;
  auto [p, ec] = std::from_chars(s.data(), s.data() + s.size(), number);
  assert(ec == std::errc());
  return number;
}

std::vector<std::string_view> SplitString(std::string_view s,
                                          char delimiter = ' ') {
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

std::string_view Trim(std::string_view s, std::string_view chars) {
  auto start = 0;
  auto end = s.size();
  while (chars.find_first_of(s[start]) != std::string::npos) {
    ++start;
  }
  while (chars.find_first_of(s[end - 1]) != std::string::npos) {
    --end;
  }
  return s.substr(start, end - start);
}
