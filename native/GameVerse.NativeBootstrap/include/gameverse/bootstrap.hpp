#pragma once

#include <cstdint>
#include <filesystem>
#include <span>
#include <string>
#include <string_view>
#include <vector>

namespace gameverse {

inline constexpr std::uint16_t kBootstrapSchema = 1;
inline constexpr std::size_t kMaximumFrame = 64 * 1024;
inline constexpr wchar_t kBootstrapPipe[] = LR"(\\.\pipe\gameverse-bootstrap-v1)";
inline constexpr std::string_view kSupportedBuild = "1.0.1158.13";
inline constexpr std::uint64_t kSupportedSize = 56'064'632;
inline constexpr std::string_view kSupportedSha256 =
    "0C52864D4521D9C9D441348AA1156958792DDE8825D0297C851753F167336401";

enum class BootstrapState {
  loaded,
  verified,
  frontend_ready,
  world_requested,
  world_ready,
  adapter_ready,
  failed,
  stopped,
};

std::string_view StateName(BootstrapState state) noexcept;

class StateMachine {
 public:
  bool Advance(BootstrapState next) noexcept;
  void Fail() noexcept;
  BootstrapState state() const noexcept { return state_; }

 private:
  BootstrapState state_{BootstrapState::loaded};
};

struct PatternByte {
  std::uint8_t value{};
  bool wildcard{};
};

std::vector<PatternByte> ParsePattern(std::string_view text);
std::vector<std::size_t> FindPattern(std::span<const std::uint8_t> bytes,
                                     std::span<const PatternByte> pattern);

struct SignatureSpec {
  std::string name;
  std::string section;
  std::vector<PatternByte> pattern;
  std::vector<std::uint8_t> prologue;
};

struct CompatibilityManifest {
  std::uint16_t schema_version{};
  std::string edition;
  std::string build;
  std::uint64_t pe_size{};
  std::string pe_sha256;
  std::string mode;
  std::vector<SignatureSpec> signatures;
};

CompatibilityManifest ParseManifest(std::string_view json);
bool VerifyManifestSignature(std::span<const std::uint8_t> manifest,
                             std::span<const std::uint8_t> signature);
std::string Sha256File(const std::filesystem::path& path);
std::string FileVersion(const std::filesystem::path& path);
bool ValidateExecutable(const std::filesystem::path& path,
                        const CompatibilityManifest& manifest,
                        std::string& error);
bool VerifyImageSignatures(void* image, const CompatibilityManifest& manifest,
                           std::string& error);

class PipeClient {
 public:
  PipeClient() = default;
  ~PipeClient();
  PipeClient(const PipeClient&) = delete;
  PipeClient& operator=(const PipeClient&) = delete;
  bool Connect(std::wstring_view pipe, std::uint32_t timeout_ms) noexcept;
  bool Send(std::string_view json) noexcept;
  bool Receive(std::string& json) noexcept;
  void Close() noexcept;

 private:
  void* handle_{reinterpret_cast<void*>(static_cast<std::intptr_t>(-1))};
};

std::string JsonEscape(std::string_view value);
std::uint64_t MonotonicMilliseconds() noexcept;
void RunBootstrap(void* module) noexcept;

}  // namespace gameverse
