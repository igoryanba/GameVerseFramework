#include "gameverse/bootstrap.hpp"

#include <windows.h>
#include <MinHook.h>

#include <algorithm>
#include <chrono>
#include <cctype>
#include <cwctype>
#include <fstream>
#include <iterator>
#include <sstream>
#include <stdexcept>
#include <thread>

namespace gameverse {
namespace {

std::vector<std::uint8_t> ReadBytes(const std::filesystem::path& path) {
  std::ifstream input(path, std::ios::binary);
  if (!input) throw std::runtime_error("file_unavailable");
  return {std::istreambuf_iterator<char>(input), std::istreambuf_iterator<char>()};
}

std::filesystem::path ModuleDirectory(void* module) {
  std::wstring buffer(32768, L'\0');
  const DWORD length = GetModuleFileNameW(static_cast<HMODULE>(module), buffer.data(), static_cast<DWORD>(buffer.size()));
  if (length == 0 || length == buffer.size()) throw std::runtime_error("module_path_unavailable");
  buffer.resize(length);
  return std::filesystem::path(buffer).parent_path();
}

std::filesystem::path ProcessPath() {
  std::wstring buffer(32768, L'\0');
  const DWORD length = GetModuleFileNameW(nullptr, buffer.data(), static_cast<DWORD>(buffer.size()));
  if (length == 0 || length == buffer.size()) throw std::runtime_error("process_path_unavailable");
  buffer.resize(length);
  return buffer;
}

bool UnsafeRuntimeDetected() {
  if (GetModuleHandleW(L"BEClient_x64.dll") || GetModuleHandleW(L"BattlEye.dll")) return true;
  std::wstring command = GetCommandLineW();
  std::transform(command.begin(), command.end(), command.begin(), [](wchar_t value) { return static_cast<wchar_t>(std::towlower(value)); });
  return command.find(L"-straightintofreemode") != std::wstring::npos ||
         command.find(L"-online") != std::wstring::npos;
}

BOOL CALLBACK FindGameWindow(HWND window, LPARAM parameter) {
  DWORD process = 0;
  GetWindowThreadProcessId(window, &process);
  if (process == GetCurrentProcessId() && IsWindowVisible(window) && !IsHungAppWindow(window)) {
    *reinterpret_cast<bool*>(parameter) = true;
    return FALSE;
  }
  return TRUE;
}

bool WaitForFrontend(std::chrono::seconds timeout) {
  const auto deadline = std::chrono::steady_clock::now() + timeout;
  while (std::chrono::steady_clock::now() < deadline) {
    bool found = false;
    EnumWindows(FindGameWindow, reinterpret_cast<LPARAM>(&found));
    if (found) return true;
    std::this_thread::sleep_for(std::chrono::milliseconds(250));
  }
  return false;
}

std::string Stage(BootstrapState state) {
  return "{\"type\":\"bootstrap_stage\",\"schema_version\":1,\"monotonic_ms\":" +
         std::to_string(MonotonicMilliseconds()) + ",\"stage\":\"" + std::string(StateName(state)) + "\"}";
}

std::string Failure(std::string_view code, std::string_view message) {
  return "{\"type\":\"bootstrap_failure\",\"schema_version\":1,\"code\":\"" +
         JsonEscape(code) + "\",\"message\":\"" + JsonEscape(message) + "\"}";
}

}  // namespace

void RunBootstrap(void* module) noexcept {
  PipeClient pipe;
  StateMachine state;
  bool minhook_initialized = false;
  try {
    if (!pipe.Connect(kBootstrapPipe, 60'000)) return;
    pipe.Send(Stage(state.state()));
    if (UnsafeRuntimeDetected()) {
      state.Fail();
      pipe.Send(Failure("unsafe_runtime_detected", "Online or BattlEye state is active"));
      return;
    }
    const auto directory = ModuleDirectory(module);
    const auto manifest_path = directory / L"enhanced-1.0.1158.13.json";
    const auto signature_path = directory / L"enhanced-1.0.1158.13.sig";
    const auto manifest_bytes = ReadBytes(manifest_path);
    const auto signature = ReadBytes(signature_path);
    if (!VerifyManifestSignature(manifest_bytes, signature)) throw std::runtime_error("manifest_signature_invalid");
    const std::string text(manifest_bytes.begin(), manifest_bytes.end());
    const auto manifest = ParseManifest(text);
    std::string validation_error;
    const auto executable = ProcessPath();
    if (!ValidateExecutable(executable, manifest, validation_error)) throw std::runtime_error(validation_error);
    if (!VerifyImageSignatures(GetModuleHandleW(nullptr), manifest, validation_error))
      throw std::runtime_error(validation_error);
    if (!state.Advance(BootstrapState::verified)) throw std::runtime_error("invalid_bootstrap_state");
    pipe.Send("{\"type\":\"bootstrap_hello\",\"schema_version\":1,\"bootstrap_build\":\"0.1.0\",\"gta_edition\":\"enhanced\",\"gta_build\":\"" +
              JsonEscape(manifest.build) + "\",\"fingerprint\":\"" + JsonEscape(manifest.pe_sha256) + "\",\"capabilities\":[\"telemetry\"]}");
    pipe.Send(Stage(state.state()));
    if (!WaitForFrontend(std::chrono::seconds(90))) throw std::runtime_error("frontend_timeout");
    if (!state.Advance(BootstrapState::frontend_ready)) throw std::runtime_error("invalid_bootstrap_state");
    pipe.Send(Stage(state.state()));

    std::string command;
    if (!pipe.Receive(command)) return;
    if (command.find("\"command\":\"abort\"") != std::string::npos ||
        command.find("\"command\":\"shutdown\"") != std::string::npos) return;
    if (command.find("\"command\":\"begin_world\"") == std::string::npos)
      throw std::runtime_error("unsupported_bootstrap_command");
    if (manifest.mode != "world_loader" || manifest.signatures.empty()) {
      state.Fail();
      pipe.Send(Failure("world_loader_unverified",
                        "No verified world-loader signatures exist for this executable fingerprint"));
      return;
    }

    // Hook installation is deliberately unreachable until every manifest signature
    // is uniquely resolved and validated inside an executable image section.
    if (MH_Initialize() != MH_OK) throw std::runtime_error("hook_runtime_initialization_failed");
    minhook_initialized = true;
    throw std::runtime_error("world_loader_not_implemented");
  } catch (const std::exception& error) {
    state.Fail();
    pipe.Send(Failure(error.what(), "Native bootstrap stopped safely"));
  } catch (...) {
    state.Fail();
    pipe.Send(Failure("bootstrap_internal_error", "Native bootstrap stopped safely"));
  }
  if (minhook_initialized) {
    MH_DisableHook(MH_ALL_HOOKS);
    MH_Uninitialize();
  }
}

}  // namespace gameverse
