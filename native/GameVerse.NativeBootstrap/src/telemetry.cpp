#include "gameverse/bootstrap.hpp"

#include <windows.h>
#include <tlhelp32.h>

#include <algorithm>
#include <array>
#include <chrono>
#include <cstring>
#include <fstream>
#include <iomanip>
#include <sstream>

namespace gameverse {
namespace {

constexpr std::size_t kMaximumModules = 96;
constexpr std::size_t kMaximumSections = 32;
constexpr std::uint32_t kMaximumSnapshots = 64;
constexpr std::uintmax_t kMaximumReportBytes = 4 * 1024 * 1024;

std::string Narrow(std::wstring_view value) {
  if (value.empty()) return {};
  const int size = WideCharToMultiByte(CP_UTF8, WC_ERR_INVALID_CHARS, value.data(),
                                       static_cast<int>(value.size()), nullptr, 0,
                                       nullptr, nullptr);
  if (size <= 0) return {};
  std::string result(static_cast<std::size_t>(size), '\0');
  WideCharToMultiByte(CP_UTF8, WC_ERR_INVALID_CHARS, value.data(),
                      static_cast<int>(value.size()), result.data(), size, nullptr,
                      nullptr);
  return result;
}

bool IsReadable(DWORD protection) {
  if ((protection & PAGE_GUARD) != 0 || (protection & PAGE_NOACCESS) != 0) return false;
  const DWORD base = protection & 0xff;
  return base == PAGE_READONLY || base == PAGE_READWRITE ||
         base == PAGE_WRITECOPY || base == PAGE_EXECUTE_READ ||
         base == PAGE_EXECUTE_READWRITE || base == PAGE_EXECUTE_WRITECOPY;
}

bool IsExecutable(DWORD protection) {
  const DWORD base = protection & 0xff;
  return base == PAGE_EXECUTE || base == PAGE_EXECUTE_READ ||
         base == PAGE_EXECUTE_READWRITE || base == PAGE_EXECUTE_WRITECOPY;
}

bool IsReadonly(DWORD protection) {
  const DWORD base = protection & 0xff;
  return base == PAGE_READONLY || base == PAGE_EXECUTE_READ;
}

std::string Bool(bool value) { return value ? "true" : "false"; }

BOOL CALLBACK InspectWindow(HWND window, LPARAM parameter) {
  DWORD process = 0;
  GetWindowThreadProcessId(window, &process);
  if (process != GetCurrentProcessId() || !IsWindowVisible(window)) return TRUE;
  auto* readiness = reinterpret_cast<TelemetryReadiness*>(parameter);
  readiness->window_visible = true;
  DWORD_PTR ignored = 0;
  readiness->window_responsive =
      SendMessageTimeoutW(window, WM_NULL, 0, 0, SMTO_ABORTIFHUNG, 100,
                          &ignored) != 0;
  return FALSE;
}

std::filesystem::path ReportPath() {
  std::array<wchar_t, 32768> local{};
  const DWORD length = GetEnvironmentVariableW(L"LOCALAPPDATA", local.data(),
                                                static_cast<DWORD>(local.size()));
  if (length == 0 || length >= local.size()) return {};
  auto directory = std::filesystem::path(local.data()) / L"GameVerse" / L"telemetry";
  std::error_code error;
  std::filesystem::create_directories(directory, error);
  if (error) return {};
  return directory /
         (L"native-" + std::to_wstring(GetCurrentProcessId()) + L"-" +
          std::to_wstring(GetTickCount64()) + L".jsonl");
}

}  // namespace

TelemetryRecorder::TelemetryRecorder(void* image)
    : image_(image), report_path_(ReportPath()) {}

TelemetrySnapshot TelemetryRecorder::Capture(std::string_view stage) {
  TelemetrySnapshot result;
  result.monotonic_ms = MonotonicMilliseconds();
  result.stage = std::string(stage.substr(0, 64));
  EnumWindows(InspectWindow, reinterpret_cast<LPARAM>(&result.readiness));
  result.readiness.scripthook_loaded = GetModuleHandleW(L"ScriptHookV.dll") != nullptr;
  result.readiness.shvdn_loaded =
      GetModuleHandleW(L"ScriptHookVDotNet.asi") != nullptr ||
      GetModuleHandleW(L"ScriptHookVDotNet2.dll") != nullptr ||
      GetModuleHandleW(L"ScriptHookVDotNet3.dll") != nullptr;
  result.readiness.adapter_loaded =
      GetModuleHandleW(L"GameVerse.GtaAdapter.dll") != nullptr ||
      GetModuleHandleW(L"GameVerse.GtaAdapter.asi") != nullptr;

  const HANDLE snapshot = CreateToolhelp32Snapshot(
      TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, GetCurrentProcessId());
  if (snapshot != INVALID_HANDLE_VALUE) {
    MODULEENTRY32W entry{};
    entry.dwSize = sizeof(entry);
    if (Module32FirstW(snapshot, &entry)) {
      do {
        TelemetryModule module;
        module.name = Narrow(entry.szModule);
        module.image_size = entry.modBaseSize;
        module.file_version = FileVersion(entry.szExePath);
        result.modules.push_back(std::move(module));
      } while (result.modules.size() < kMaximumModules &&
               Module32NextW(snapshot, &entry));
    }
    CloseHandle(snapshot);
  }
  std::sort(result.modules.begin(), result.modules.end(),
            [](const auto& left, const auto& right) { return left.name < right.name; });

  const auto* base = static_cast<const std::uint8_t*>(image_);
  if (!base) return result;
  const auto* dos = reinterpret_cast<const IMAGE_DOS_HEADER*>(base);
  if (dos->e_magic != IMAGE_DOS_SIGNATURE || dos->e_lfanew <= 0) return result;
  const auto* nt = reinterpret_cast<const IMAGE_NT_HEADERS64*>(base + dos->e_lfanew);
  if (nt->Signature != IMAGE_NT_SIGNATURE ||
      nt->OptionalHeader.Magic != IMAGE_NT_OPTIONAL_HDR64_MAGIC)
    return result;
  const auto* first = IMAGE_FIRST_SECTION(nt);
  const auto section_count =
      std::min<std::size_t>(nt->FileHeader.NumberOfSections, kMaximumSections);
  const auto page_size = [] {
    SYSTEM_INFO info{};
    GetSystemInfo(&info);
    return static_cast<std::size_t>(info.dwPageSize);
  }();
  for (std::size_t index = 0; index < section_count; ++index) {
    const auto& source = first[index];
    char name[IMAGE_SIZEOF_SHORT_NAME + 1]{};
    std::memcpy(name, source.Name, IMAGE_SIZEOF_SHORT_NAME);
    TelemetrySection section;
    section.name = name;
    section.virtual_size = source.Misc.VirtualSize;
    section.characteristics = source.Characteristics;
    std::vector<std::uint8_t> page_digests;
    const auto* section_base = base + source.VirtualAddress;
    const auto section_size = static_cast<std::size_t>(source.Misc.VirtualSize);
    for (std::size_t offset = 0; offset < section_size; offset += page_size) {
      MEMORY_BASIC_INFORMATION memory{};
      if (VirtualQuery(section_base + offset, &memory, sizeof(memory)) != sizeof(memory) ||
          memory.State != MEM_COMMIT || !IsReadable(memory.Protect))
        continue;
      ++section.committed_pages;
      if (IsExecutable(memory.Protect)) ++section.executable_pages;
      if (IsReadonly(memory.Protect)) ++section.readonly_pages;
      const auto length = std::min(page_size, section_size - offset);
      const auto page_hash = Sha256Bytes(std::span(section_base + offset, length));
      const auto key = section.name + ":" + std::to_string(offset / page_size);
      const auto previous = page_hashes_.find(key);
      if (previous != page_hashes_.end() && previous->second != page_hash)
        ++section.changed_pages;
      page_hashes_[key] = page_hash;
      for (std::size_t byte = 0; byte < page_hash.size(); byte += 2) {
        page_digests.push_back(static_cast<std::uint8_t>(
            std::stoul(page_hash.substr(byte, 2), nullptr, 16)));
      }
    }
    section.aggregate_sha256 =
        Sha256Bytes(std::span<const std::uint8_t>(page_digests));
    result.sections.push_back(std::move(section));
  }
  return result;
}

bool TelemetryRecorder::AppendLocal(const TelemetrySnapshot& snapshot) noexcept {
  if (report_path_.empty() || snapshots_ >= kMaximumSnapshots) return false;
  try {
    std::error_code error;
    if (std::filesystem::exists(report_path_, error) &&
        std::filesystem::file_size(report_path_, error) >= kMaximumReportBytes)
      return false;
    std::ofstream output(report_path_, std::ios::binary | std::ios::app);
    if (!output) return false;
    output << SerializeTelemetrySnapshot(snapshot) << '\n';
    ++snapshots_;
    return output.good();
  } catch (...) {
    return false;
  }
}

std::string SerializeTelemetrySnapshot(const TelemetrySnapshot& value) {
  std::string output =
      "{\"type\":\"telemetry_snapshot_v1\",\"schema_version\":1,\"snapshot\":{"
      "\"monotonic_ms\":" + std::to_string(value.monotonic_ms) +
      ",\"stage\":\"" + JsonEscape(value.stage) + "\",\"modules\":[";
  for (std::size_t index = 0; index < value.modules.size(); ++index) {
    if (index != 0) output += ',';
    const auto& module = value.modules[index];
    output += "{\"name\":\"" + JsonEscape(module.name) +
              "\",\"image_size\":" + std::to_string(module.image_size) +
              ",\"file_version\":\"" + JsonEscape(module.file_version) + "\"}";
  }
  output += "],\"sections\":[";
  for (std::size_t index = 0; index < value.sections.size(); ++index) {
    if (index != 0) output += ',';
    const auto& section = value.sections[index];
    output += "{\"name\":\"" + JsonEscape(section.name) +
              "\",\"virtual_size\":" + std::to_string(section.virtual_size) +
              ",\"characteristics\":" + std::to_string(section.characteristics) +
              ",\"committed_pages\":" + std::to_string(section.committed_pages) +
              ",\"executable_pages\":" + std::to_string(section.executable_pages) +
              ",\"readonly_pages\":" + std::to_string(section.readonly_pages) +
              ",\"changed_pages\":" + std::to_string(section.changed_pages) +
              ",\"aggregate_sha256\":\"" + section.aggregate_sha256 + "\"}";
  }
  const auto& ready = value.readiness;
  output += "],\"readiness\":{\"window_visible\":" + Bool(ready.window_visible) +
            ",\"window_responsive\":" + Bool(ready.window_responsive) +
            ",\"scripthook_loaded\":" + Bool(ready.scripthook_loaded) +
            ",\"shvdn_loaded\":" + Bool(ready.shvdn_loaded) +
            ",\"adapter_loaded\":" + Bool(ready.adapter_loaded) + "}}}";
  return output;
}

}  // namespace gameverse
