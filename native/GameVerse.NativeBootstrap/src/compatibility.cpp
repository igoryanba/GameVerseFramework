#include "gameverse/bootstrap.hpp"

#include "gameverse/public_key.hpp"

#include <windows.h>
#include <bcrypt.h>

#include <algorithm>
#include <array>
#include <charconv>
#include <stdexcept>

namespace gameverse {
namespace {

std::string StringField(std::string_view json, std::string_view name) {
  const std::string key = "\"" + std::string(name) + "\"";
  auto position = json.find(key);
  if (position == std::string_view::npos) throw std::invalid_argument("missing manifest field");
  position = json.find(':', position + key.size());
  if (position == std::string_view::npos) throw std::invalid_argument("invalid manifest field");
  position = json.find('"', position + 1);
  if (position == std::string_view::npos) throw std::invalid_argument("invalid string field");
  const auto end = json.find('"', position + 1);
  if (end == std::string_view::npos) throw std::invalid_argument("unterminated string field");
  return std::string(json.substr(position + 1, end - position - 1));
}

std::uint64_t IntegerField(std::string_view json, std::string_view name) {
  const std::string key = "\"" + std::string(name) + "\"";
  auto position = json.find(key);
  if (position == std::string_view::npos) throw std::invalid_argument("missing manifest number");
  position = json.find(':', position + key.size());
  if (position == std::string_view::npos) throw std::invalid_argument("invalid manifest number");
  do { ++position; } while (position < json.size() && (json[position] == ' ' || json[position] == '\r' || json[position] == '\n'));
  std::uint64_t result = 0;
  const auto parsed = std::from_chars(json.data() + position, json.data() + json.size(), result);
  if (parsed.ec != std::errc{}) throw std::invalid_argument("invalid integer field");
  return result;
}

bool ConstantTimeEqual(std::string_view left, std::string_view right) noexcept {
  if (left.size() != right.size()) return false;
  unsigned difference = 0;
  for (std::size_t index = 0; index < left.size(); ++index)
    difference |= static_cast<unsigned>(left[index] ^ right[index]);
  return difference == 0;
}

}  // namespace

CompatibilityManifest ParseManifest(std::string_view json) {
  CompatibilityManifest manifest;
  manifest.schema_version = static_cast<std::uint16_t>(IntegerField(json, "schema_version"));
  manifest.edition = StringField(json, "edition");
  manifest.build = StringField(json, "build");
  manifest.pe_size = IntegerField(json, "pe_size");
  manifest.pe_sha256 = StringField(json, "pe_sha256");
  manifest.mode = StringField(json, "mode");
  if (manifest.schema_version != 1 || manifest.edition != "enhanced" ||
      manifest.pe_sha256.size() != 64 ||
      (manifest.mode != "telemetry_only" && manifest.mode != "world_loader"))
    throw std::invalid_argument("unsupported compatibility manifest");
  const auto signatures_key = json.find("\"signatures\"");
  if (signatures_key == std::string_view::npos) throw std::invalid_argument("missing signatures");
  const auto array_begin = json.find('[', signatures_key);
  const auto array_end = json.find(']', array_begin);
  if (array_begin == std::string_view::npos || array_end == std::string_view::npos)
    throw std::invalid_argument("invalid signatures array");
  auto cursor = array_begin + 1;
  while ((cursor = json.find('{', cursor)) != std::string_view::npos && cursor < array_end) {
    const auto object_end = json.find('}', cursor);
    if (object_end == std::string_view::npos || object_end > array_end)
      throw std::invalid_argument("invalid signature object");
    const auto object = json.substr(cursor, object_end - cursor + 1);
    SignatureSpec spec;
    spec.name = StringField(object, "name");
    spec.section = StringField(object, "section");
    spec.pattern = ParsePattern(StringField(object, "pattern"));
    const auto prologue = ParsePattern(StringField(object, "prologue"));
    if (prologue.size() > spec.pattern.size() ||
        std::any_of(prologue.begin(), prologue.end(),
                    [](const PatternByte& value) { return value.wildcard; }))
      throw std::invalid_argument("invalid signature prologue");
    for (const auto value : prologue) spec.prologue.push_back(value.value);
    if (spec.name.empty() || spec.name.size() > 64 || spec.section != ".text" ||
        std::any_of(manifest.signatures.begin(), manifest.signatures.end(),
                    [&](const SignatureSpec& existing) { return existing.name == spec.name; }))
      throw std::invalid_argument("invalid signature identity");
    manifest.signatures.push_back(std::move(spec));
    cursor = object_end + 1;
  }
  if (manifest.signatures.size() > 16 ||
      (manifest.mode == "world_loader" && manifest.signatures.empty()))
    throw std::invalid_argument("world loader requires verified signatures");
  return manifest;
}

bool VerifyManifestSignature(std::span<const std::uint8_t> manifest,
                             std::span<const std::uint8_t> signature) {
  if (signature.size() != 64) return false;
  BCRYPT_ALG_HANDLE algorithm = nullptr;
  BCRYPT_KEY_HANDLE key = nullptr;
  BCRYPT_HASH_HANDLE hash = nullptr;
  std::array<std::uint8_t, 32> digest{};
  std::vector<std::uint8_t> object;
  DWORD object_size = 0;
  DWORD copied = 0;
  bool ok = false;
  if (BCryptOpenAlgorithmProvider(&algorithm, BCRYPT_ECDSA_P256_ALGORITHM, nullptr, 0) < 0) goto cleanup;
  {
    struct PublicBlob {
      BCRYPT_ECCKEY_BLOB header;
      std::array<std::uint8_t, 64> coordinates;
    } blob{{BCRYPT_ECDSA_PUBLIC_P256_MAGIC, 32}, kManifestPublicKey};
    if (BCryptImportKeyPair(algorithm, nullptr, BCRYPT_ECCPUBLIC_BLOB, &key,
                            reinterpret_cast<PUCHAR>(&blob), sizeof(blob), 0) < 0) goto cleanup;
  }
  BCryptCloseAlgorithmProvider(algorithm, 0);
  algorithm = nullptr;
  if (BCryptOpenAlgorithmProvider(&algorithm, BCRYPT_SHA256_ALGORITHM, nullptr, 0) < 0) goto cleanup;
  if (BCryptGetProperty(algorithm, BCRYPT_OBJECT_LENGTH, reinterpret_cast<PUCHAR>(&object_size),
                        sizeof(object_size), &copied, 0) < 0) goto cleanup;
  object.resize(object_size);
  if (BCryptCreateHash(algorithm, &hash, object.data(), static_cast<ULONG>(object.size()), nullptr, 0, 0) < 0) goto cleanup;
  if (BCryptHashData(hash, const_cast<PUCHAR>(manifest.data()), static_cast<ULONG>(manifest.size()), 0) < 0) goto cleanup;
  if (BCryptFinishHash(hash, digest.data(), static_cast<ULONG>(digest.size()), 0) < 0) goto cleanup;
  ok = BCryptVerifySignature(key, nullptr, digest.data(), static_cast<ULONG>(digest.size()),
                             const_cast<PUCHAR>(signature.data()), static_cast<ULONG>(signature.size()), 0) >= 0;
cleanup:
  if (hash) BCryptDestroyHash(hash);
  if (key) BCryptDestroyKey(key);
  if (algorithm) BCryptCloseAlgorithmProvider(algorithm, 0);
  return ok;
}

bool ValidateExecutable(const std::filesystem::path& path,
                        const CompatibilityManifest& manifest,
                        std::string& error) {
  std::error_code ec;
  const auto size = std::filesystem::file_size(path, ec);
  if (ec || size != manifest.pe_size || size != kSupportedSize) {
    error = "unsupported_executable_size";
    return false;
  }
  const auto version = FileVersion(path);
  if (!ConstantTimeEqual(version, manifest.build) || !ConstantTimeEqual(version, kSupportedBuild)) {
    error = "unsupported_game_build";
    return false;
  }
  const auto digest = Sha256File(path);
  if (!ConstantTimeEqual(digest, manifest.pe_sha256) || !ConstantTimeEqual(digest, kSupportedSha256)) {
    error = "unsupported_executable_fingerprint";
    return false;
  }
  return true;
}

}  // namespace gameverse
