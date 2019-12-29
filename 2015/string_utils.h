#include <charconv>
#include <string_view>

template <typename T> T sv2number(std::string_view s) {
  T number;
  auto [p, ec] = std::from_chars(s.data(), s.data() + s.size(), number);
  assert(ec == std::errc());
  return number;
}

