#include "gameverse/bootstrap.hpp"

#include <windows.h>

#include <array>
#include <algorithm>
#include <iostream>
#include <fstream>
#include <iterator>
#include <stdexcept>
#include <thread>

namespace {
void Require(bool condition, const char* message) {
  if (!condition) throw std::runtime_error(message);
}

#pragma optimize("", off)
__declspec(noinline) int ObserveTarget(int value) {
  volatile int result = value;
  for (volatile int index = 0; index < 7; ++index) result += index * 3;
  return result;
}
volatile std::uint32_t gSyntheticInitState = 1;
#pragma optimize("", on)
}

int main() {
  try {
    const auto pattern = gameverse::ParsePattern("48 8B ?? 90");
    const std::array<std::uint8_t, 10> bytes{0x48, 0x8b, 0x01, 0x90, 0xcc, 0x48, 0x8b, 0x02, 0x90, 0xcc};
    const auto matches = gameverse::FindPattern(bytes, pattern);
    Require(matches.size() == 2 && matches[0] == 0 && matches[1] == 5, "wildcard or duplicate scan failed");
    Require(gameverse::FindPattern(std::span(bytes).subspan(0, 4), gameverse::ParsePattern("48 8B 01 90")).size() == 1,
            "unique scan failed");
    Require(gameverse::FindPattern(bytes, gameverse::ParsePattern("DE AD BE EF")).empty(), "missing scan failed");
    bool malformed = false;
    try { static_cast<void>(gameverse::ParsePattern("4Z")); } catch (const std::invalid_argument&) { malformed = true; }
    Require(malformed, "malformed pattern accepted");

    const std::wstring delayed_pipe =
        LR"(\\.\pipe\gameverse-bootstrap-delayed-test-)" +
        std::to_wstring(GetCurrentProcessId());
    std::thread server([&] {
      Sleep(150);
      const HANDLE handle = CreateNamedPipeW(
          delayed_pipe.c_str(), PIPE_ACCESS_DUPLEX, PIPE_TYPE_BYTE | PIPE_WAIT,
          1, 4096, 4096, 0, nullptr);
      if (handle != INVALID_HANDLE_VALUE) {
        static_cast<void>(ConnectNamedPipe(handle, nullptr));
        CloseHandle(handle);
      }
    });
    gameverse::PipeClient delayed_client;
    Require(delayed_client.Connect(delayed_pipe, 2'000),
            "pipe client did not wait for a delayed server");
    delayed_client.Close();
    server.join();

    gameverse::StateMachine state;
    Require(!state.Advance(gameverse::BootstrapState::world_ready), "state skip accepted");
    Require(state.Advance(gameverse::BootstrapState::verified), "verified rejected");
    Require(state.Advance(gameverse::BootstrapState::frontend_ready), "frontend rejected");
    state.Fail();
    Require(!state.Advance(gameverse::BootstrapState::world_requested), "failed state advanced");

    const auto manifest = gameverse::ParseManifest(
        R"({"schema_version":1,"edition":"enhanced","build":"1.0.1158.13","pe_size":56064632,"pe_sha256":"0C52864D4521D9C9D441348AA1156958792DDE8825D0297C851753F167336401","mode":"telemetry_only","signatures":[]})");
    Require(manifest.schema_version == 1 && manifest.signatures.empty(), "manifest parse failed");
    const auto with_signature = gameverse::ParseManifest(
        R"({"schema_version":1,"edition":"enhanced","build":"1.0.1158.13","pe_size":56064632,"pe_sha256":"0C52864D4521D9C9D441348AA1156958792DDE8825D0297C851753F167336401","mode":"world_loader","signatures":[{"name":"world_request","section":".text","pattern":"48 8B ?? 90","prologue":"48 8B"}]})");
    Require(with_signature.signatures.size() == 1 &&
                with_signature.signatures[0].prologue.size() == 2,
            "signature manifest parse failed");
    const auto with_hash_signature = gameverse::ParseManifest(
        R"({"schema_version":1,"edition":"enhanced","build":"1.0.1158.13","pe_size":56064632,"pe_sha256":"0C52864D4521D9C9D441348AA1156958792DDE8825D0297C851753F167336401","mode":"observe_only","signatures":[{"name":"caller","section":".text","rva":4096,"entry_sha256":"1111111111111111111111111111111111111111111111111111111111111111"}]})");
    Require(with_hash_signature.signatures.size() == 1 &&
                with_hash_signature.signatures[0].pattern.empty() &&
                with_hash_signature.signatures[0].rva == std::uint32_t{4096},
            "hash-only signature manifest parse failed");
    bool incomplete_hash_signature = false;
    try {
      static_cast<void>(gameverse::ParseManifest(
          R"({"schema_version":1,"edition":"enhanced","build":"1.0.1158.13","pe_size":56064632,"pe_sha256":"0C52864D4521D9C9D441348AA1156958792DDE8825D0297C851753F167336401","mode":"observe_only","signatures":[{"name":"caller","section":".text","rva":4096}]})"));
    } catch (const std::invalid_argument&) {
      incomplete_hash_signature = true;
    }
    Require(incomplete_hash_signature, "hash-only signature without digest accepted");
    bool unsafe_prologue = false;
    try {
      static_cast<void>(gameverse::ParseManifest(
          R"({"schema_version":1,"edition":"enhanced","build":"1.0.1158.13","pe_size":56064632,"pe_sha256":"0C52864D4521D9C9D441348AA1156958792DDE8825D0297C851753F167336401","mode":"world_loader","signatures":[{"name":"world_request","section":".text","pattern":"48 8B ?? 90","prologue":"48 ??"}]})"));
    } catch (const std::invalid_argument&) {
      unsafe_prologue = true;
    }
    Require(unsafe_prologue, "wildcard prologue accepted");
    const auto read = [](const wchar_t* path) {
      std::ifstream input(std::filesystem::path(path), std::ios::binary);
      return std::vector<std::uint8_t>(std::istreambuf_iterator<char>(input), {});
    };
    const auto signed_manifest = read(GAMEVERSE_TEST_MANIFEST);
    auto signature = read(GAMEVERSE_TEST_SIGNATURE);
    Require(gameverse::VerifyManifestSignature(signed_manifest, signature), "manifest signature rejected");
    signature[0] ^= 1;
    Require(!gameverse::VerifyManifestSignature(signed_manifest, signature), "damaged signature accepted");
    gameverse::TelemetryRecorder telemetry(GetModuleHandleW(nullptr));
    const auto telemetry_snapshot = telemetry.Capture("synthetic_host");
    Require(!telemetry_snapshot.modules.empty(), "telemetry module inventory empty");
    Require(!telemetry_snapshot.sections.empty(), "telemetry section inventory empty");
    const auto telemetry_json = gameverse::SerializeTelemetrySnapshot(telemetry_snapshot);
    Require(telemetry_json.find("telemetry_snapshot_v1") != std::string::npos,
            "telemetry serialization failed");
    Require(telemetry_json.find("0x") == std::string::npos,
            "telemetry leaked an absolute address");
    const auto candidate_manifest_bytes = read(GAMEVERSE_TEST_CANDIDATES);
    const auto candidate_signature = read(GAMEVERSE_TEST_CANDIDATE_SIGNATURE);
    Require(gameverse::VerifyManifestSignature(candidate_manifest_bytes,
                                                candidate_signature),
            "observe-only manifest signature rejected");
    const auto candidate_manifest = gameverse::ParseManifest(
        std::string(candidate_manifest_bytes.begin(), candidate_manifest_bytes.end()));
    Require(candidate_manifest.mode == "telemetry_only" &&
                candidate_manifest.signatures.empty(),
            "rejected research candidates remain active");
    const auto candidates = gameverse::InspectImageCandidates(
        GetModuleHandleW(nullptr), candidate_manifest);
    Require(candidates.size() == candidate_manifest.signatures.size(),
            "research candidates were not inspected");
    const auto candidates_json = gameverse::SerializeTelemetryCandidates(candidates);
    Require(candidates_json.find("telemetry_candidates_v1") != std::string::npos,
            "candidate telemetry serialization failed");
    Require(candidates_json.find("0x") == std::string::npos,
            "candidate telemetry leaked an absolute address");
    Require(gameverse::TelemetryPageKey(0, ".text", 1) !=
                gameverse::TelemetryPageKey(1, ".text", 1),
            "duplicate PE section names share a telemetry page key");
    gameverse::InitStateSampler state_sampler(GetModuleHandleW(nullptr));
    Require(state_sampler.Start("synthetic_transition"),
            "init state sampler did not start");
    gSyntheticInitState = 2;
    Require(state_sampler.Poll(), "init state sampler did not poll");
    gSyntheticInitState = 7;
    Require(state_sampler.Poll(), "init state sampler second poll failed");
    const auto state_candidates = state_sampler.Finish();
    const auto synthetic_rva = static_cast<std::uint32_t>(
        reinterpret_cast<std::uintptr_t>(&gSyntheticInitState) -
        reinterpret_cast<std::uintptr_t>(GetModuleHandleW(nullptr)));
    const auto found_state = std::find_if(
        state_candidates.begin(), state_candidates.end(), [&](const auto& value) {
          return value.rva == synthetic_rva && value.transition_count == 2 &&
                 value.distinct_state_count == 3;
        });
    Require(found_state != state_candidates.end(),
            "synthetic init state transition was not captured");
    const auto state_json =
        gameverse::SerializeInitStateCandidates(state_candidates);
    Require(state_json.find("init_state_candidates_v1") != std::string::npos &&
                state_json.find("synthetic_transition") != std::string::npos,
            "init state candidate serialization failed");
    Require(state_json.find("0x") == std::string::npos,
            "init state telemetry leaked an absolute address");
    const auto adapter_log = std::filesystem::temp_directory_path() /
                             L"gameverse-native-adapter-test.log";
    std::ofstream(adapter_log) << "old run\n";
    const auto initial_size = std::filesystem::file_size(adapter_log);
    Require(!gameverse::AdapterLogContainsMarkerAfter(adapter_log, initial_size),
            "unchanged managed adapter log was accepted");
    std::ofstream(adapter_log, std::ios::app)
        << "GTA_ADAPTER_LOADED=true BUILD=1.0.1158.13\n";
    Require(gameverse::AdapterLogContainsMarkerAfter(adapter_log, initial_size),
            "current managed adapter log was not detected");
    std::filesystem::remove(adapter_log);

    gameverse::CompatibilityManifest observe_manifest;
    observe_manifest.schema_version = 1;
    observe_manifest.edition = "enhanced";
    observe_manifest.mode = "observe_only";
    gameverse::SignatureSpec observe_signature;
    observe_signature.name = "synthetic_observe_target";
    observe_signature.section = ".text";
    const auto* observe_bytes = reinterpret_cast<const std::uint8_t*>(&ObserveTarget);
    for (std::size_t index = 0; index < 32; ++index)
      observe_signature.pattern.push_back({observe_bytes[index], false});
    observe_signature.entry_sha256 =
        gameverse::Sha256Bytes(std::span(observe_bytes, std::size_t{32}));
    observe_manifest.signatures.push_back(std::move(observe_signature));
    std::vector<gameverse::TelemetryCandidate> observed;
    std::string observe_error;
    gameverse::ObserveHookSession observe_session;
    if (!observe_session.Start(GetModuleHandleW(nullptr), observe_manifest,
                               observed, observe_error))
      throw std::runtime_error(observe_error);
    Require(ObserveTarget(5) == 68, "observe hook changed function result");
    observe_session.Refresh(observed);
    Require(observed.size() == 1 && observed[0].call_count == 1,
            "observe hook did not count exactly one call");
    const auto callers = gameverse::InspectDirectCallers(
        GetModuleHandleW(nullptr), observed);
    const auto callers_json = gameverse::SerializeTelemetryCallers(callers);
    Require(callers_json.find("telemetry_callers_v1") != std::string::npos,
            "direct caller telemetry serialization failed");
    Require(callers_json.find("0x") == std::string::npos,
            "direct caller telemetry leaked an absolute address");
    std::cout << "native bootstrap tests passed\n";
    return 0;
  } catch (const std::exception& error) {
    std::cerr << error.what() << '\n';
    return 1;
  }
}
