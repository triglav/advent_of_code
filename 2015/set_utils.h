#pragma once

#include <algorithm>
#include <unordered_map>
#include <vector>

template <typename T> std::vector<T> Without(std::vector<T> v, T value) {
  auto it = std::remove(v.begin(), v.end(), value);
  v.erase(it, v.end());
  return v;
}

template <typename T> std::vector<T> With(std::vector<T> v, T value) {
  v.push_back(value);
  return v;
}

template <typename K, typename V>
std::vector<K> Keys(std::unordered_map<K, V> const &map) {
  std::vector<K> keys;
  for (auto const &[k, v] : map) {
    keys.push_back(k);
  }
  return keys;
}

namespace detail {
template <typename T>
void VariationStep(std::vector<T> const &items, std::vector<T> const &current,
                   std::vector<std::vector<T>> *variations) {
  if (items.empty()) {
    variations->push_back(current);
    return;
  }

  for (auto const &i : items) {
    VariationStep(Without(items, i), With(current, i), variations);
  }
}
} // namespace detail

template <typename T>
std::vector<std::vector<T>> GetVariations(std::vector<T> const &all_items) {
  std::vector<std::vector<T>> v;
  detail::VariationStep(all_items, {}, &v);
  return v;
}

template <typename T>
std::vector<std::vector<T>>
GetVariationsRound(std::vector<T> const &all_items) {
  std::vector<std::vector<T>> v;
  auto first = all_items.front();
  detail::VariationStep(Without(all_items, first), {first}, &v);
  return v;
}

