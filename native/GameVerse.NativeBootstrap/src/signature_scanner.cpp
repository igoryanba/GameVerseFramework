#include "gameverse/bootstrap.hpp"

#include <charconv>
#include <stdexcept>

namespace gameverse {

std::vector<PatternByte> ParsePattern(std::string_view text) {
  std::vector<PatternByte> result;
  std::size_t cursor = 0;
  while (cursor < text.size()) {
    while (cursor < text.size() && text[cursor] == ' ') ++cursor;
    if (cursor == text.size()) break;
    const auto end = text.find(' ', cursor);
    const auto token = text.substr(cursor, end == std::string_view::npos ? text.size() - cursor : end - cursor);
    if (token == "?" || token == "??") {
      result.push_back({0, true});
    } else {
      if (token.size() != 2) throw std::invalid_argument("invalid signature token");
      unsigned value = 0;
      const auto parsed = std::from_chars(token.data(), token.data() + token.size(), value, 16);
      if (parsed.ec != std::errc{} || parsed.ptr != token.data() + token.size() || value > 0xff)
        throw std::invalid_argument("invalid signature byte");
      result.push_back({static_cast<std::uint8_t>(value), false});
    }
    cursor = end == std::string_view::npos ? text.size() : end + 1;
  }
  if (result.empty()) throw std::invalid_argument("empty signature");
  return result;
}

std::vector<std::size_t> FindPattern(std::span<const std::uint8_t> bytes,
                                     std::span<const PatternByte> pattern) {
  std::vector<std::size_t> matches;
  if (pattern.empty() || pattern.size() > bytes.size()) return matches;
  for (std::size_t offset = 0; offset <= bytes.size() - pattern.size(); ++offset) {
    bool equal = true;
    for (std::size_t index = 0; index < pattern.size(); ++index) {
      if (!pattern[index].wildcard && pattern[index].value != bytes[offset + index]) {
        equal = false;
        break;
      }
    }
    if (equal) matches.push_back(offset);
  }
  return matches;
}

}  // namespace gameverse
