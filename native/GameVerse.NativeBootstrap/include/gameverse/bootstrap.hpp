#pragma once

#include <cstdint>
#include <filesystem>
#include <span>
#include <string>
#include <string_view>
#include <unordered_map>
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
std::string Sha256Bytes(std::span<const std::uint8_t> bytes);
std::string FileVersion(const std::filesystem::path& path);
bool ValidateExecutable(const std::filesystem::path& path,
                        const CompatibilityManifest& manifest,
                        std::string& error);
bool VerifyImageSignatures(void* image, const CompatibilityManifest& manifest,
                           std::string& error);

struct TelemetryCandidate {
  std::string candidate_id;
  std::uint32_t rva{};
  std::string section;
  std::uint32_t unique_match_count{};
  std::uint64_t call_count{};
  std::string entry_sha256;
};

std::vector<TelemetryCandidate> InspectImageCandidates(
    void* image, const CompatibilityManifest& manifest);
std::string SerializeTelemetryCandidates(
    std::span<const TelemetryCandidate> candidates);

struct TelemetryModule {
  std::string name;
  std::uint64_t image_size{};
  std::string file_version;
};

struct TelemetrySection {
  std::string name;
  std::uint64_t virtual_size{};
  std::uint32_t characteristics{};
  std::uint32_t committed_pages{};
  std::uint32_t executable_pages{};
  std::uint32_t readonly_pages{};
  std::uint32_t changed_pages{};
  std::vector<std::uint32_t> changed_page_rvas;
  std::string aggregate_sha256;
};

struct TelemetryReadiness {
  bool window_visible{};
  bool window_responsive{};
  bool scripthook_loaded{};
  bool shvdn_loaded{};
  bool adapter_loaded{};
};

struct TelemetrySnapshot {
  std::uint64_t monotonic_ms{};
  std::string stage;
  std::vector<TelemetryModule> modules;
  std::vector<TelemetrySection> sections;
  TelemetryReadiness readiness;
};

std::string TelemetryPageKey(std::size_t section_index,
                             std::string_view section_name,
                             std::size_t page_index);
bool AdapterLogContainsMarkerAfter(const std::filesystem::path& path,
                                   std::uintmax_t initial_size) noexcept;

class TelemetryRecorder {
 public:
  explicit TelemetryRecorder(void* image);
  TelemetryReadiness ObserveReadiness() const noexcept;
  TelemetrySnapshot Capture(std::string_view stage);
  bool AppendLocal(const TelemetrySnapshot& snapshot) noexcept;
  const std::filesystem::path& report_path() const noexcept { return report_path_; }

 private:
  void* image_{};
  std::unordered_map<std::string, std::string> page_hashes_;
  std::filesystem::path report_path_;
  std::filesystem::path adapter_log_path_;
  std::uintmax_t adapter_log_initial_size_{};
  std::uint32_t snapshots_{};
};

std::string SerializeTelemetrySnapshot(const TelemetrySnapshot& snapshot);

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
void AppendStartupDiagnostic(std::string_view event) noexcept;

}  // namespace gameverse
