#include "gameverse/bootstrap.hpp"

#include <windows.h>

#include <array>
#include <iostream>
#include <fstream>
#include <iterator>
#include <stdexcept>

namespace {
void Require(bool condition, const char* message) {
  if (!condition) throw std::runtime_error(message);
}
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
    const auto candidate_manifest = gameverse::ParseManifest(
        std::string(candidate_manifest_bytes.begin(), candidate_manifest_bytes.end()));
    const auto candidates = gameverse::InspectImageCandidates(
        GetModuleHandleW(nullptr), candidate_manifest);
    Require(candidates.size() == 2, "research candidates were not inspected");
    const auto candidates_json = gameverse::SerializeTelemetryCandidates(candidates);
    Require(candidates_json.find("telemetry_candidates_v1") != std::string::npos,
            "candidate telemetry serialization failed");
    Require(candidates_json.find("0x") == std::string::npos,
            "candidate telemetry leaked an absolute address");
    Require(gameverse::TelemetryPageKey(0, ".text", 1) !=
                gameverse::TelemetryPageKey(1, ".text", 1),
            "duplicate PE section names share a telemetry page key");
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
    std::cout << "native bootstrap tests passed\n";
    return 0;
  } catch (const std::exception& error) {
    std::cerr << error.what() << '\n';
    return 1;
  }
}
