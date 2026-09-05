#include "gameverse/bootstrap.hpp"

#include <windows.h>
#include <bcrypt.h>

#include <array>
#include <fstream>
#include <iomanip>
#include <limits>
#include <sstream>
#include <stdexcept>
#include <vector>

namespace gameverse {

namespace {
std::string Sha256Chunks(const std::vector<std::span<const std::uint8_t>>& chunks) {
  BCRYPT_ALG_HANDLE algorithm = nullptr;
  BCRYPT_HASH_HANDLE hash = nullptr;
  DWORD object_size = 0, copied = 0;
  std::vector<std::uint8_t> object;
  std::array<std::uint8_t, 32> digest{};
  if (BCryptOpenAlgorithmProvider(&algorithm, BCRYPT_SHA256_ALGORITHM, nullptr, 0) < 0)
    throw std::runtime_error("sha256 unavailable");
  if (BCryptGetProperty(algorithm, BCRYPT_OBJECT_LENGTH,
                        reinterpret_cast<PUCHAR>(&object_size), sizeof(object_size),
                        &copied, 0) < 0)
    goto failure;
  object.resize(object_size);
  if (BCryptCreateHash(algorithm, &hash, object.data(),
                       static_cast<ULONG>(object.size()), nullptr, 0, 0) < 0)
    goto failure;
  for (const auto chunk : chunks) {
    if (chunk.size() > static_cast<std::size_t>(std::numeric_limits<ULONG>::max()) ||
        BCryptHashData(hash, const_cast<PUCHAR>(chunk.data()),
                       static_cast<ULONG>(chunk.size()), 0) < 0)
      goto failure;
  }
  if (BCryptFinishHash(hash, digest.data(), static_cast<ULONG>(digest.size()), 0) < 0)
    goto failure;
  BCryptDestroyHash(hash);
  BCryptCloseAlgorithmProvider(algorithm, 0);
  {
    std::ostringstream output;
    output << std::uppercase << std::hex << std::setfill('0');
    for (auto byte : digest) output << std::setw(2) << static_cast<unsigned>(byte);
    return output.str();
  }
failure:
  if (hash) BCryptDestroyHash(hash);
  if (algorithm) BCryptCloseAlgorithmProvider(algorithm, 0);
  throw std::runtime_error("sha256 failed");
}
}  // namespace

std::string Sha256Bytes(std::span<const std::uint8_t> bytes) {
  return Sha256Chunks({bytes});
}

std::string Sha256File(const std::filesystem::path& path) {
  std::ifstream input(path, std::ios::binary);
  if (!input) throw std::runtime_error("cannot open executable");
  BCRYPT_ALG_HANDLE algorithm = nullptr;
  BCRYPT_HASH_HANDLE hash = nullptr;
  DWORD object_size = 0, copied = 0;
  std::vector<std::uint8_t> object;
  std::array<std::uint8_t, 32> digest{};
  if (BCryptOpenAlgorithmProvider(&algorithm, BCRYPT_SHA256_ALGORITHM, nullptr, 0) < 0) throw std::runtime_error("sha256 unavailable");
  if (BCryptGetProperty(algorithm, BCRYPT_OBJECT_LENGTH, reinterpret_cast<PUCHAR>(&object_size), sizeof(object_size), &copied, 0) < 0) goto failure;
  object.resize(object_size);
  if (BCryptCreateHash(algorithm, &hash, object.data(), static_cast<ULONG>(object.size()), nullptr, 0, 0) < 0) goto failure;
  {
    std::array<char, 1024 * 1024> buffer{};
    while (input) {
      input.read(buffer.data(), buffer.size());
      const auto count = input.gcount();
      if (count > 0 && BCryptHashData(hash, reinterpret_cast<PUCHAR>(buffer.data()), static_cast<ULONG>(count), 0) < 0) goto failure;
    }
  }
  if (BCryptFinishHash(hash, digest.data(), static_cast<ULONG>(digest.size()), 0) < 0) goto failure;
  BCryptDestroyHash(hash);
  BCryptCloseAlgorithmProvider(algorithm, 0);
  {
    std::ostringstream output;
    output << std::uppercase << std::hex << std::setfill('0');
    for (auto byte : digest) output << std::setw(2) << static_cast<unsigned>(byte);
    return output.str();
  }
failure:
  if (hash) BCryptDestroyHash(hash);
  if (algorithm) BCryptCloseAlgorithmProvider(algorithm, 0);
  throw std::runtime_error("sha256 failed");
}

std::string FileVersion(const std::filesystem::path& path) {
  DWORD ignored = 0;
  const DWORD size = GetFileVersionInfoSizeW(path.c_str(), &ignored);
  if (size == 0) return {};
  std::vector<std::uint8_t> data(size);
  if (!GetFileVersionInfoW(path.c_str(), 0, size, data.data())) return {};
  VS_FIXEDFILEINFO* info = nullptr;
  UINT info_size = 0;
  if (!VerQueryValueW(data.data(), L"\\", reinterpret_cast<void**>(&info), &info_size) || info_size < sizeof(*info)) return {};
  std::ostringstream result;
  result << HIWORD(info->dwFileVersionMS) << '.' << LOWORD(info->dwFileVersionMS) << '.'
         << HIWORD(info->dwFileVersionLS) << '.' << LOWORD(info->dwFileVersionLS);
  return result.str();
}

}  // namespace gameverse
