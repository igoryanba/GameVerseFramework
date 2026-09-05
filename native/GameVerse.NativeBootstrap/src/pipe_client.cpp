#include "gameverse/bootstrap.hpp"

#include <windows.h>

#include <array>

namespace gameverse {

PipeClient::~PipeClient() { Close(); }

bool PipeClient::Connect(std::wstring_view pipe, std::uint32_t timeout_ms) noexcept {
  Close();
  const std::wstring name(pipe);
  if (!WaitNamedPipeW(name.c_str(), timeout_ms)) return false;
  HANDLE value = CreateFileW(name.c_str(), GENERIC_READ | GENERIC_WRITE, 0, nullptr,
                             OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, nullptr);
  if (value == INVALID_HANDLE_VALUE) return false;
  handle_ = value;
  return true;
}

bool PipeClient::Send(std::string_view json) noexcept {
  if (handle_ == INVALID_HANDLE_VALUE || json.empty() || json.size() > kMaximumFrame) return false;
  std::array<std::uint8_t, 4> prefix{
      static_cast<std::uint8_t>(json.size() >> 24), static_cast<std::uint8_t>(json.size() >> 16),
      static_cast<std::uint8_t>(json.size() >> 8), static_cast<std::uint8_t>(json.size())};
  DWORD written = 0;
  if (!WriteFile(handle_, prefix.data(), static_cast<DWORD>(prefix.size()), &written, nullptr) || written != prefix.size()) return false;
  return WriteFile(handle_, json.data(), static_cast<DWORD>(json.size()), &written, nullptr) && written == json.size();
}

bool PipeClient::Receive(std::string& json) noexcept {
  if (handle_ == INVALID_HANDLE_VALUE) return false;
  std::array<std::uint8_t, 4> prefix{};
  DWORD read = 0;
  if (!ReadFile(handle_, prefix.data(), static_cast<DWORD>(prefix.size()), &read, nullptr) || read != prefix.size()) return false;
  const std::uint32_t length = (static_cast<std::uint32_t>(prefix[0]) << 24) |
      (static_cast<std::uint32_t>(prefix[1]) << 16) | (static_cast<std::uint32_t>(prefix[2]) << 8) | prefix[3];
  if (length == 0 || length > kMaximumFrame) return false;
  json.resize(length);
  std::size_t offset = 0;
  while (offset < length) {
    if (!ReadFile(handle_, json.data() + offset, length - static_cast<DWORD>(offset), &read, nullptr) || read == 0) return false;
    offset += read;
  }
  return true;
}

void PipeClient::Close() noexcept {
  if (handle_ != INVALID_HANDLE_VALUE) CloseHandle(handle_);
  handle_ = INVALID_HANDLE_VALUE;
}

std::string JsonEscape(std::string_view value) {
  std::string result;
  result.reserve(value.size());
  for (const char character : value) {
    switch (character) {
      case '"': result += "\\\""; break;
      case '\\': result += "\\\\"; break;
      case '\n': result += "\\n"; break;
      case '\r': result += "\\r"; break;
      case '\t': result += "\\t"; break;
      default:
        if (static_cast<unsigned char>(character) >= 0x20) result += character;
    }
  }
  return result;
}

std::uint64_t MonotonicMilliseconds() noexcept { return GetTickCount64(); }

}  // namespace gameverse
