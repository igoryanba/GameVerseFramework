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

std::string ObservedStage(std::string_view stage) {
  return "{\"type\":\"bootstrap_stage\",\"schema_version\":1,\"monotonic_ms\":" +
         std::to_string(MonotonicMilliseconds()) + ",\"stage\":\"" +
         JsonEscape(stage) + "\"}";
}

std::string Failure(std::string_view code, std::string_view message) {
  return "{\"type\":\"bootstrap_failure\",\"schema_version\":1,\"code\":\"" +
         JsonEscape(code) + "\",\"message\":\"" + JsonEscape(message) + "\"}";
}

std::string JsonStringField(std::string_view json, std::string_view name) {
  const auto key = "\"" + std::string(name) + "\"";
  auto position = json.find(key);
  if (position == std::string_view::npos) return {};
  position = json.find(':', position + key.size());
  if (position == std::string_view::npos) return {};
  position = json.find('"', position + 1);
  if (position == std::string_view::npos) return {};
  const auto end = json.find('"', position + 1);
  if (end == std::string_view::npos) return {};
  auto value = std::string(json.substr(position + 1, end - position - 1));
  if (value.empty() || value.size() > 64 ||
      !std::all_of(value.begin(), value.end(), [](unsigned char character) {
        return std::isalnum(character) != 0 || character == '_' || character == '-';
      }))
    return {};
  return value;
}

std::optional<std::uint32_t> JsonU32Field(std::string_view json,
                                         std::string_view name) {
  const auto key = "\"" + std::string(name) + "\"";
  auto position = json.find(key);
  if (position == std::string_view::npos) return std::nullopt;
  position = json.find(':', position + key.size());
  if (position == std::string_view::npos) return std::nullopt;
  ++position;
  while (position < json.size() &&
         std::isspace(static_cast<unsigned char>(json[position])) != 0)
    ++position;
  const auto begin = position;
  std::uint64_t value = 0;
  while (position < json.size() &&
         std::isdigit(static_cast<unsigned char>(json[position])) != 0) {
    value = value * 10 + static_cast<unsigned>(json[position] - '0');
    if (value > UINT32_MAX) return std::nullopt;
    ++position;
  }
  if (position == begin) return std::nullopt;
  while (position < json.size() &&
         std::isspace(static_cast<unsigned char>(json[position])) != 0)
    ++position;
  if (position >= json.size() || (json[position] != ',' && json[position] != '}'))
    return std::nullopt;
  return static_cast<std::uint32_t>(value);
}

bool SendInitStateCandidateBatches(
    PipeClient& pipe, TelemetryRecorder& telemetry,
    std::span<const InitStateCandidate> candidates) {
  constexpr std::size_t kBatchSize = 160;
  if (candidates.empty()) {
    const auto json = SerializeInitStateCandidates(candidates);
    if (!telemetry.AppendLocalJson(json) || !pipe.Send(json)) return false;
  } else {
    for (std::size_t offset = 0; offset < candidates.size(); offset += kBatchSize) {
      const auto batch = candidates.subspan(
          offset, std::min(kBatchSize, candidates.size() - offset));
      const auto json = SerializeInitStateCandidates(batch);
      if (!telemetry.AppendLocalJson(json) || !pipe.Send(json)) return false;
    }
  }
  const auto done =
      "{\"type\":\"init_state_candidates_done_v1\",\"schema_version\":1,"
      "\"total_count\":" + std::to_string(candidates.size()) + "}";
  return telemetry.AppendLocalJson(done) && pipe.Send(done);
}

}  // namespace

void AppendStartupDiagnostic(std::string_view event) noexcept {
  try {
    std::wstring local_app_data(32768, L'\0');
    const auto length = GetEnvironmentVariableW(
        L"LOCALAPPDATA", local_app_data.data(),
        static_cast<DWORD>(local_app_data.size()));
    if (length == 0 || length >= local_app_data.size()) return;
    local_app_data.resize(length);
    const auto directory = std::filesystem::path(local_app_data) / L"GameVerse" /
                           L"telemetry";
    std::filesystem::create_directories(directory);
    const auto path = directory / L"bootstrap-startup.log";
    std::error_code error;
    if (std::filesystem::exists(path, error) &&
        std::filesystem::file_size(path, error) > 64 * 1024)
      std::filesystem::resize_file(path, 0, error);
    std::ofstream output(path, std::ios::binary | std::ios::app);
    output << MonotonicMilliseconds() << ' ' << event << '\n';
  } catch (...) {
  }
}

void RunBootstrap(void* module) noexcept {
  PipeClient pipe;
  StateMachine state;
  ObserveHookSession observe_hooks;
  std::vector<TelemetryCandidate> observed_candidates;
  bool minhook_initialized = false;
  try {
    AppendStartupDiagnostic("bootstrap_entered");
    if (!pipe.Connect(kBootstrapPipe, 60'000)) {
      AppendStartupDiagnostic("pipe_unavailable");
      return;
    }
    AppendStartupDiagnostic("pipe_connected");
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
    TelemetryRecorder telemetry(GetModuleHandleW(nullptr));
    const std::string bootstrap_capabilities =
        manifest.mode == "world_loader" ? "[\"telemetry\",\"world_loader\"]"
                                        : "[\"telemetry\"]";
    pipe.Send("{\"type\":\"bootstrap_hello\",\"schema_version\":1,\"bootstrap_build\":\"0.1.0\",\"gta_edition\":\"enhanced\",\"gta_build\":\"" +
              JsonEscape(manifest.build) + "\",\"fingerprint\":\"" +
              JsonEscape(manifest.pe_sha256) + "\",\"capabilities\":" +
              bootstrap_capabilities + "}");
    pipe.Send(Stage(state.state()));
    pipe.Send("{\"type\":\"telemetry_hello_v1\",\"schema_version\":1,\"probe_build\":\"0.1.0\",\"gta_build\":\"" +
              JsonEscape(manifest.build) + "\",\"fingerprint\":\"" +
              JsonEscape(manifest.pe_sha256) +
              "\",\"capabilities\":[\"pe_sections\",\"module_inventory\",\"page_hashes\"]}");
    std::string command;
    if (!pipe.Receive(command)) return;
    if (command.find("\"command\":\"abort\"") != std::string::npos ||
        command.find("\"command\":\"shutdown\"") != std::string::npos)
      return;
    if (command.find("\"command\":\"start_telemetry\"") == std::string::npos)
      throw std::runtime_error("telemetry_start_required");
    auto snapshot = telemetry.Capture("image_verified");
    telemetry.AppendLocal(snapshot);
    if (!pipe.Send(SerializeTelemetrySnapshot(snapshot)))
      throw std::runtime_error("telemetry_frame_rejected");
    if (!WaitForFrontend(std::chrono::seconds(90))) throw std::runtime_error("frontend_timeout");
    if (!state.Advance(BootstrapState::frontend_ready)) throw std::runtime_error("invalid_bootstrap_state");
    snapshot = telemetry.Capture("frontend_stable");
    telemetry.AppendLocal(snapshot);
    if (!pipe.Send(SerializeTelemetrySnapshot(snapshot)))
      throw std::runtime_error("telemetry_frame_rejected");
    pipe.Send(Stage(state.state()));

    if (manifest.mode == "telemetry_only") {
      InitStateSampler state_sampler(GetModuleHandleW(nullptr));
      const auto candidates_path = directory / L"telemetry-candidates-v1.json";
      if (std::filesystem::exists(candidates_path)) {
      const auto candidate_bytes = ReadBytes(candidates_path);
      const auto candidate_signature =
          ReadBytes(directory / L"telemetry-candidates-v1.sig");
      if (!VerifyManifestSignature(candidate_bytes, candidate_signature))
        throw std::runtime_error("telemetry_candidates_signature_invalid");
      const std::string candidate_text(candidate_bytes.begin(), candidate_bytes.end());
      const auto candidate_manifest = ParseManifest(candidate_text);
        if ((candidate_manifest.mode != "telemetry_only" &&
             candidate_manifest.mode != "observe_only") ||
            candidate_manifest.edition != manifest.edition ||
            candidate_manifest.build != manifest.build ||
            candidate_manifest.pe_size != manifest.pe_size ||
            candidate_manifest.pe_sha256 != manifest.pe_sha256)
          throw std::runtime_error("telemetry_candidates_identity_mismatch");
        if (candidate_manifest.mode == "observe_only") {
          observed_candidates =
              InspectImageCandidates(GetModuleHandleW(nullptr), candidate_manifest);
          const auto caller_candidates = InspectDirectCallers(
              GetModuleHandleW(nullptr), observed_candidates);
          std::string observe_error;
          if (!observe_hooks.Start(GetModuleHandleW(nullptr), candidate_manifest,
                                   observed_candidates, observe_error))
            throw std::runtime_error(observe_error);
          if (!pipe.Send(SerializeTelemetryCallers(caller_candidates)))
            throw std::runtime_error("telemetry_callers_frame_rejected");
        } else {
          observed_candidates =
              InspectImageCandidates(GetModuleHandleW(nullptr), candidate_manifest);
          const auto caller_candidates = InspectDirectCallers(
              GetModuleHandleW(nullptr), observed_candidates);
          if (!pipe.Send(SerializeTelemetryCallers(caller_candidates)))
            throw std::runtime_error("telemetry_callers_frame_rejected");
        }
        if (!pipe.Send(SerializeTelemetryCandidates(observed_candidates)))
          throw std::runtime_error("telemetry_candidates_frame_rejected");
      }
      auto previous = telemetry.ObserveReadiness();
      auto next_candidate_sample =
          std::chrono::steady_clock::now() + std::chrono::seconds(5);
      const auto deadline = std::chrono::steady_clock::now() + std::chrono::minutes(15);
      while (std::chrono::steady_clock::now() < deadline) {
        std::this_thread::sleep_for(std::chrono::milliseconds(50));
        std::string live_command;
        if (pipe.TryReceive(live_command)) {
          if (live_command.find("\"command\":\"abort\"") != std::string::npos ||
              live_command.find("\"command\":\"shutdown\"") != std::string::npos)
            return;
          if (live_command.find("\"type\":\"telemetry_marker_v1\"") !=
              std::string::npos) {
            const auto marker_id = JsonStringField(live_command, "marker_id");
            if (marker_id.empty())
              throw std::runtime_error("invalid_telemetry_marker");
            if (!state_sampler.Start(marker_id))
              throw std::runtime_error("init_state_sampler_unavailable");
            const auto marker = SerializeTelemetryMarker(
                marker_id, MonotonicMilliseconds());
            telemetry.AppendLocal(telemetry.Capture("marker_" + marker_id));
            telemetry.AppendLocalJson(marker);
            if (!pipe.Send(marker)) return;
          } else if (live_command.find("\"type\":\"state_writer_probe_v1\"") !=
                     std::string::npos) {
            const auto state_rva = JsonU32Field(live_command, "state_rva");
            if (!state_rva || *state_rva == 0 || (*state_rva % 4) != 0)
              throw std::runtime_error("invalid_state_writer_probe");
            const auto writers =
                InspectStateWriters(GetModuleHandleW(nullptr), *state_rva);
            const auto writers_json = SerializeStateWriters(writers);
            telemetry.AppendLocalJson(writers_json);
            if (!pipe.Send(writers_json)) return;
          } else if (live_command.find("\"command\":\"finish_telemetry\"") !=
                     std::string::npos) {
            if (state_sampler.active()) {
              const auto candidates = state_sampler.Finish();
              if (!SendInitStateCandidateBatches(pipe, telemetry, candidates))
                return;
            }
          } else {
            throw std::runtime_error("unsupported_bootstrap_command");
          }
        }
        if (state_sampler.active()) {
          static_cast<void>(state_sampler.Poll());
          if (state_sampler.expired()) {
            const auto candidates = state_sampler.Finish();
            if (!SendInitStateCandidateBatches(pipe, telemetry, candidates))
              return;
          }
        }
        if (!observed_candidates.empty() &&
            std::chrono::steady_clock::now() >= next_candidate_sample) {
          observe_hooks.Refresh(observed_candidates);
          if (!pipe.Send(SerializeTelemetryCandidates(observed_candidates)))
            return;
          next_candidate_sample =
              std::chrono::steady_clock::now() + std::chrono::seconds(5);
        }
        const auto current = telemetry.ObserveReadiness();
        const bool runtime_changed =
            current.scripthook_loaded != previous.scripthook_loaded ||
            current.shvdn_loaded != previous.shvdn_loaded;
        if (runtime_changed) {
          snapshot = telemetry.Capture("world_transition");
          telemetry.AppendLocal(snapshot);
          if (!pipe.Send(SerializeTelemetrySnapshot(snapshot))) return;
        }
        if (current.adapter_loaded && !previous.adapter_loaded) {
          // A managed SHVDN adapter starts only after the local world exists.
          // Preserve that transition as a distinct trace stage while reusing a
          // single bounded PE snapshot for both observations.
          snapshot = telemetry.Capture("world_transition");
          telemetry.AppendLocal(snapshot);
          if (!pipe.Send(SerializeTelemetrySnapshot(snapshot))) return;
          observe_hooks.Refresh(observed_candidates);
          if (!observed_candidates.empty() &&
              !pipe.Send(SerializeTelemetryCandidates(observed_candidates)))
            return;
          if (state_sampler.active()) {
            const auto candidates = state_sampler.Finish();
            if (!SendInitStateCandidateBatches(pipe, telemetry, candidates))
              return;
          }
          snapshot.stage = "adapter_loaded";
          telemetry.AppendLocal(snapshot);
          if (!pipe.Send(SerializeTelemetrySnapshot(snapshot))) return;
          pipe.Send(ObservedStage("adapter_ready"));
          return;
        }
        previous = current;
      }
      pipe.Send(Failure("telemetry_adapter_timeout",
                        "Adapter did not appear during the telemetry window"));
      return;
    }

    if (!pipe.Receive(command)) return;
    if (command.find("\"command\":\"abort\"") != std::string::npos ||
        command.find("\"command\":\"shutdown\"") != std::string::npos) return;
    if (command.find("\"command\":\"begin_world\"") == std::string::npos)
      throw std::runtime_error("unsupported_bootstrap_command");
    snapshot = telemetry.Capture("world_transition");
    telemetry.AppendLocal(snapshot);
    if (!pipe.Send(SerializeTelemetrySnapshot(snapshot)))
      throw std::runtime_error("telemetry_frame_rejected");
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
    AppendStartupDiagnostic(error.what());
    state.Fail();
    pipe.Send(Failure(error.what(), "Native bootstrap stopped safely"));
  } catch (...) {
    AppendStartupDiagnostic("bootstrap_internal_error");
    state.Fail();
    pipe.Send(Failure("bootstrap_internal_error", "Native bootstrap stopped safely"));
  }
  if (minhook_initialized) {
    MH_DisableHook(MH_ALL_HOOKS);
    MH_Uninitialize();
  }
}

}  // namespace gameverse
