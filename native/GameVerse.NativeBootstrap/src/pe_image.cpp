#include "gameverse/bootstrap.hpp"

#include <windows.h>

#include <algorithm>
#include <cstring>
#include <map>

namespace gameverse {

namespace {

bool ReadImage(void* image, const std::uint8_t*& base,
               const IMAGE_NT_HEADERS64*& nt) {
  if (!image) return false;
  base = static_cast<const std::uint8_t*>(image);
  const auto dos = reinterpret_cast<const IMAGE_DOS_HEADER*>(base);
  if (dos->e_magic != IMAGE_DOS_SIGNATURE || dos->e_lfanew <= 0) return false;
  nt = reinterpret_cast<const IMAGE_NT_HEADERS64*>(base + dos->e_lfanew);
  return nt->Signature == IMAGE_NT_SIGNATURE &&
         nt->OptionalHeader.Magic == IMAGE_NT_OPTIONAL_HDR64_MAGIC;
}

std::string SectionName(const IMAGE_SECTION_HEADER& section) {
  char name[IMAGE_SIZEOF_SHORT_NAME + 1]{};
  std::memcpy(name, section.Name, IMAGE_SIZEOF_SHORT_NAME);
  return name;
}

}  // namespace

bool VerifyImageSignatures(void* image, const CompatibilityManifest& manifest,
                           std::string& error) {
  const std::uint8_t* base = nullptr;
  const IMAGE_NT_HEADERS64* nt = nullptr;
  if (!ReadImage(image, base, nt)) {
    error = "invalid_pe_image";
    return false;
  }
  const auto first = IMAGE_FIRST_SECTION(nt);
  for (const auto& spec : manifest.signatures) {
    std::vector<std::size_t> matches;
    const IMAGE_SECTION_HEADER* matched_section = nullptr;
    for (unsigned index = 0; index < nt->FileHeader.NumberOfSections; ++index) {
      const auto& section = first[index];
      if (spec.section != SectionName(section)) continue;
      if ((section.Characteristics & IMAGE_SCN_MEM_EXECUTE) == 0) {
        error = "signature_section_not_executable";
        return false;
      }
      const auto size = static_cast<std::size_t>(section.Misc.VirtualSize);
      const auto section_matches =
          FindPattern(std::span(base + section.VirtualAddress, size), spec.pattern);
      if (!section_matches.empty()) {
        if (matched_section != nullptr) {
          error = "signature_not_unique";
          return false;
        }
        matches = section_matches;
        matched_section = &section;
      }
    }
    if (!matched_section || matches.size() != 1) {
      error = matches.empty() ? "signature_not_found" : "signature_not_unique";
      return false;
    }
    const auto address = base + matched_section->VirtualAddress + matches.front();
    if (!std::equal(spec.prologue.begin(), spec.prologue.end(), address)) {
      error = "signature_prologue_mismatch";
      return false;
    }
  }
  return true;
}

std::vector<TelemetryCandidate> InspectImageCandidates(
    void* image, const CompatibilityManifest& manifest) {
  std::vector<TelemetryCandidate> result;
  const std::uint8_t* base = nullptr;
  const IMAGE_NT_HEADERS64* nt = nullptr;
  if (!ReadImage(image, base, nt)) return result;
  const auto first = IMAGE_FIRST_SECTION(nt);
  for (const auto& spec : manifest.signatures) {
    TelemetryCandidate candidate{spec.name, 0, spec.section, 0, 0, {}};
    std::uint32_t matched_rva = 0;
    for (unsigned index = 0; index < nt->FileHeader.NumberOfSections; ++index) {
      const auto& section = first[index];
      if (spec.section != SectionName(section) ||
          (section.Characteristics & IMAGE_SCN_MEM_EXECUTE) == 0)
        continue;
      const auto size = static_cast<std::size_t>(section.Misc.VirtualSize);
      const auto matches =
          FindPattern(std::span(base + section.VirtualAddress, size), spec.pattern);
      candidate.unique_match_count += static_cast<std::uint32_t>(matches.size());
      if (matches.size() == 1 && candidate.unique_match_count == 1)
        matched_rva = section.VirtualAddress + static_cast<std::uint32_t>(matches.front());
    }
    if (candidate.unique_match_count == 1) {
      DWORD64 image_base = reinterpret_cast<DWORD64>(base);
      const auto function = RtlLookupFunctionEntry(
          reinterpret_cast<DWORD64>(base + matched_rva), &image_base, nullptr);
      candidate.rva = function != nullptr && image_base == reinterpret_cast<DWORD64>(base)
                          ? function->BeginAddress
                          : matched_rva;
      if (candidate.rva <= nt->OptionalHeader.SizeOfImage - 32)
        candidate.entry_sha256 =
            Sha256Bytes(std::span(base + candidate.rva, std::size_t{32}));
    }
    result.push_back(std::move(candidate));
  }
  return result;
}

std::vector<TelemetryCallerCandidate> InspectDirectCallers(
    void* image, std::span<const TelemetryCandidate> candidates) {
  std::vector<TelemetryCallerCandidate> result;
  const std::uint8_t* base = nullptr;
  const IMAGE_NT_HEADERS64* nt = nullptr;
  if (!ReadImage(image, base, nt)) return result;
  const auto image_base = reinterpret_cast<std::uintptr_t>(base);
  const auto image_end = image_base + nt->OptionalHeader.SizeOfImage;
  const auto first = IMAGE_FIRST_SECTION(nt);
  for (const auto& candidate : candidates) {
    if (candidate.rva == 0 || candidate.unique_match_count != 1) continue;
    const auto target = image_base + candidate.rva;
    std::map<std::uint32_t, std::uint32_t> callers;
    for (unsigned index = 0; index < nt->FileHeader.NumberOfSections; ++index) {
      const auto& section = first[index];
      if ((section.Characteristics & IMAGE_SCN_MEM_EXECUTE) == 0) continue;
      const auto section_begin = base + section.VirtualAddress;
      const auto section_size = static_cast<std::size_t>(section.Misc.VirtualSize);
      if (section_size < 5) continue;
      for (std::size_t offset = 0; offset <= section_size - 5; ++offset) {
        if (section_begin[offset] != 0xe8) continue;
        std::int32_t displacement = 0;
        std::memcpy(&displacement, section_begin + offset + 1,
                    sizeof(displacement));
        const auto call_address = reinterpret_cast<std::uintptr_t>(section_begin + offset);
        if (call_address + 5 + displacement != target) continue;
        DWORD64 lookup_base = static_cast<DWORD64>(image_base);
        const auto entry = RtlLookupFunctionEntry(
            static_cast<DWORD64>(call_address), &lookup_base, nullptr);
        if (!entry || lookup_base != image_base ||
            entry->BeginAddress >= nt->OptionalHeader.SizeOfImage) continue;
        ++callers[entry->BeginAddress];
      }
    }
    for (const auto& [caller_rva, call_sites] : callers) {
      if (result.size() >= 128) return result;
      if (image_base + caller_rva + 32 > image_end) continue;
      result.push_back(
          {candidate.candidate_id, caller_rva, call_sites,
           Sha256Bytes(std::span(base + caller_rva, std::size_t{32}))});
    }
  }
  return result;
}

}  // namespace gameverse
