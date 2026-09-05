#include "gameverse/bootstrap.hpp"

#include <windows.h>

#include <algorithm>
#include <cstring>

namespace gameverse {

bool VerifyImageSignatures(void* image, const CompatibilityManifest& manifest,
                           std::string& error) {
  if (!image) {
    error = "image_unavailable";
    return false;
  }
  const auto base = static_cast<const std::uint8_t*>(image);
  const auto dos = reinterpret_cast<const IMAGE_DOS_HEADER*>(base);
  if (dos->e_magic != IMAGE_DOS_SIGNATURE || dos->e_lfanew <= 0) {
    error = "invalid_pe_image";
    return false;
  }
  const auto nt = reinterpret_cast<const IMAGE_NT_HEADERS64*>(base + dos->e_lfanew);
  if (nt->Signature != IMAGE_NT_SIGNATURE ||
      nt->OptionalHeader.Magic != IMAGE_NT_OPTIONAL_HDR64_MAGIC) {
    error = "invalid_pe_image";
    return false;
  }
  const auto first = IMAGE_FIRST_SECTION(nt);
  for (const auto& spec : manifest.signatures) {
    std::vector<std::size_t> matches;
    const IMAGE_SECTION_HEADER* matched_section = nullptr;
    for (unsigned index = 0; index < nt->FileHeader.NumberOfSections; ++index) {
      const auto& section = first[index];
      char name[IMAGE_SIZEOF_SHORT_NAME + 1]{};
      std::memcpy(name, section.Name, IMAGE_SIZEOF_SHORT_NAME);
      if (spec.section != name) continue;
      if ((section.Characteristics & IMAGE_SCN_MEM_EXECUTE) == 0) {
        error = "signature_section_not_executable";
        return false;
      }
      const auto size = static_cast<std::size_t>(section.Misc.VirtualSize);
      matches = FindPattern(std::span(base + section.VirtualAddress, size), spec.pattern);
      matched_section = &section;
      break;
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

}  // namespace gameverse
