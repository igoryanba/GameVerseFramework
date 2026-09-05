#include "gameverse/bootstrap.hpp"

#include <windows.h>
#include <MinHook.h>

#include <array>
#include <cstring>

namespace gameverse {
namespace {

void WriteDisplacement(std::uint8_t* destination, std::uintptr_t from,
                       std::uintptr_t to) {
  const auto value = static_cast<std::int32_t>(
      to - from);
  std::memcpy(destination, &value, sizeof(value));
}

}  // namespace

ObserveHookSession::~ObserveHookSession() { Stop(); }

bool ObserveHookSession::Start(void* image,
                               const CompatibilityManifest& manifest,
                               std::vector<TelemetryCandidate>& candidates,
                               std::string& error) {
  Stop();
  candidates = InspectImageCandidates(image, manifest);
  if (candidates.size() != manifest.signatures.size()) {
    error = "observe_candidate_count_mismatch";
    return false;
  }
  for (std::size_t index = 0; index < candidates.size(); ++index) {
    if (candidates[index].unique_match_count != 1 || candidates[index].rva == 0 ||
        manifest.signatures[index].entry_sha256.empty() ||
        candidates[index].entry_sha256 != manifest.signatures[index].entry_sha256) {
      error = "observe_candidate_attestation_failed";
      return false;
    }
  }
  if (MH_Initialize() != MH_OK) {
    error = "observe_hook_runtime_initialization_failed";
    return false;
  }
  initialized_ = true;
  const auto base = static_cast<std::uint8_t*>(image);
  for (const auto& candidate : candidates) {
    auto* allocation = static_cast<std::uint8_t*>(
        VirtualAlloc(nullptr, 8192, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE));
    if (!allocation) {
      error = "observe_hook_allocation_failed";
      Stop();
      return false;
    }
    auto* trampoline_slot = reinterpret_cast<void**>(allocation + 4096);
    auto* counter = reinterpret_cast<volatile long long*>(allocation + 4104);
    const std::array<std::uint8_t, 38> stub{
        0x9c, 0x50, 0x51, 0x52, 0x41, 0x50, 0x41, 0x51, 0x41, 0x52,
        0x41, 0x53, 0xf0, 0x48, 0xff, 0x05, 0,    0,    0,    0,
        0x41, 0x5b, 0x41, 0x5a, 0x41, 0x59, 0x41, 0x58, 0x5a, 0x59,
        0x58, 0x9d, 0xff, 0x25, 0,    0,    0,    0};
    std::memcpy(allocation, stub.data(), stub.size());
    WriteDisplacement(allocation + 16,
                      reinterpret_cast<std::uintptr_t>(allocation + 20),
                      reinterpret_cast<std::uintptr_t>(counter));
    WriteDisplacement(allocation + 34,
                      reinterpret_cast<std::uintptr_t>(allocation + 38),
                      reinterpret_cast<std::uintptr_t>(trampoline_slot));
    DWORD previous = 0;
    if (!VirtualProtect(allocation, 4096, PAGE_EXECUTE_READ, &previous) ||
        !FlushInstructionCache(GetCurrentProcess(), allocation, 4096)) {
      VirtualFree(allocation, 0, MEM_RELEASE);
      error = "observe_hook_code_protection_failed";
      Stop();
      return false;
    }
    void* trampoline = nullptr;
    void* target = base + candidate.rva;
    const auto create_status = MH_CreateHook(target, allocation, &trampoline);
    if (create_status != MH_OK) {
      VirtualFree(allocation, 0, MEM_RELEASE);
      error = "observe_hook_creation_failed";
      Stop();
      return false;
    }
    *trampoline_slot = trampoline;
    if (MH_EnableHook(target) != MH_OK) {
      static_cast<void>(MH_RemoveHook(target));
      VirtualFree(allocation, 0, MEM_RELEASE);
      error = "observe_hook_enable_failed";
      Stop();
      return false;
    }
    hooks_.push_back({target, allocation, counter});
  }
  return true;
}

void ObserveHookSession::Refresh(
    std::vector<TelemetryCandidate>& candidates) const noexcept {
  const auto count = (std::min)(candidates.size(), hooks_.size());
  for (std::size_t index = 0; index < count; ++index)
    candidates[index].call_count = static_cast<std::uint64_t>(
        InterlockedCompareExchange64(hooks_[index].counter, 0, 0));
}

void ObserveHookSession::Stop() noexcept {
  for (auto iterator = hooks_.rbegin(); iterator != hooks_.rend(); ++iterator) {
    static_cast<void>(MH_DisableHook(iterator->target));
    static_cast<void>(MH_RemoveHook(iterator->target));
    VirtualFree(iterator->allocation, 0, MEM_RELEASE);
  }
  hooks_.clear();
  if (initialized_) {
    static_cast<void>(MH_Uninitialize());
    initialized_ = false;
  }
}

}  // namespace gameverse
