#include "gameverse/bootstrap.hpp"

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
    const auto read = [](const wchar_t* path) {
      std::ifstream input(std::filesystem::path(path), std::ios::binary);
      return std::vector<std::uint8_t>(std::istreambuf_iterator<char>(input), {});
    };
    const auto signed_manifest = read(GAMEVERSE_TEST_MANIFEST);
    auto signature = read(GAMEVERSE_TEST_SIGNATURE);
    Require(gameverse::VerifyManifestSignature(signed_manifest, signature), "manifest signature rejected");
    signature[0] ^= 1;
    Require(!gameverse::VerifyManifestSignature(signed_manifest, signature), "damaged signature accepted");
    std::cout << "native bootstrap tests passed\n";
    return 0;
  } catch (const std::exception& error) {
    std::cerr << error.what() << '\n';
    return 1;
  }
}
