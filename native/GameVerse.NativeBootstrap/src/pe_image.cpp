#include "gameverse/bootstrap.hpp"

#include <windows.h>
#include <Zydis/Zydis.h>

#include <algorithm>
#include <cstring>
#include <map>
#include <set>

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
    if (spec.rva) {
      const IMAGE_SECTION_HEADER* selected = nullptr;
      for (unsigned index = 0; index < nt->FileHeader.NumberOfSections; ++index) {
        const auto& section = first[index];
        const auto begin = section.VirtualAddress;
        const auto end = static_cast<std::uint64_t>(begin) + section.Misc.VirtualSize;
        if (spec.section == SectionName(section) && *spec.rva >= begin &&
            static_cast<std::uint64_t>(*spec.rva) + 32 <= end)
          selected = &section;
      }
      if (!selected || (selected->Characteristics & IMAGE_SCN_MEM_EXECUTE) == 0) {
        error = "signature_rva_not_executable";
        return false;
      }
      const auto candidate = base + *spec.rva;
      if (Sha256Bytes(std::span(candidate, std::size_t{32})) !=
          spec.entry_sha256) {
        error = "signature_entry_hash_mismatch";
        return false;
      }
      std::uint32_t matches = 0;
      const auto section_bytes = base + selected->VirtualAddress;
      const auto section_size = static_cast<std::size_t>(selected->Misc.VirtualSize);
      if (section_size < 32) {
        error = "signature_section_too_small";
        return false;
      }
      for (std::size_t offset = 0; offset <= section_size - 32; ++offset)
        if (std::memcmp(section_bytes + offset, candidate, 32) == 0) ++matches;
      if (matches != 1) {
        error = "signature_hash_not_unique";
        return false;
      }
      continue;
    }
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
    if (spec.rva) {
      const IMAGE_SECTION_HEADER* selected = nullptr;
      for (unsigned index = 0; index < nt->FileHeader.NumberOfSections; ++index) {
        const auto& section = first[index];
        const auto begin = section.VirtualAddress;
        const auto end = static_cast<std::uint64_t>(begin) + section.Misc.VirtualSize;
        if (spec.section == SectionName(section) && *spec.rva >= begin &&
            static_cast<std::uint64_t>(*spec.rva) + 32 <= end &&
            (section.Characteristics & IMAGE_SCN_MEM_EXECUTE) != 0)
          selected = &section;
      }
      if (selected) {
        const auto entry = base + *spec.rva;
        const auto section_bytes = base + selected->VirtualAddress;
        const auto section_size = static_cast<std::size_t>(selected->Misc.VirtualSize);
        if (section_size < 32) {
          result.push_back(std::move(candidate));
          continue;
        }
        for (std::size_t offset = 0; offset <= section_size - 32; ++offset)
          if (std::memcmp(section_bytes + offset, entry, 32) == 0)
            ++candidate.unique_match_count;
        candidate.rva = *spec.rva;
        candidate.entry_sha256 =
            Sha256Bytes(std::span(entry, std::size_t{32}));
      }
      result.push_back(std::move(candidate));
      continue;
    }
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
  ZydisDecoder decoder;
  if (!ZYAN_SUCCESS(ZydisDecoderInit(&decoder, ZYDIS_MACHINE_MODE_LONG_64,
                                      ZYDIS_STACK_WIDTH_64)))
    return result;
  struct WorkItem {
    std::string candidate_id;
    std::uint32_t rva;
    std::uint32_t depth;
  };
  std::vector<WorkItem> work;
  std::set<std::uint32_t> visited;
  for (const auto& candidate : candidates) {
    if (candidate.rva == 0 || candidate.unique_match_count != 1) continue;
    if (visited.insert(candidate.rva).second)
      work.push_back({candidate.candidate_id, candidate.rva, 0});
  }
  for (std::size_t work_index = 0;
       work_index < work.size() && result.size() < 128; ++work_index) {
    const auto item = work[work_index];
    const auto target = image_base + item.rva;
    std::map<std::uint32_t, std::uint32_t> callers;
    for (unsigned index = 0; index < nt->FileHeader.NumberOfSections; ++index) {
      const auto& section = first[index];
      if ((section.Characteristics & IMAGE_SCN_MEM_EXECUTE) == 0) continue;
      const auto section_begin = base + section.VirtualAddress;
      const auto section_size = static_cast<std::size_t>(section.Misc.VirtualSize);
      if (section_size < 5) continue;
      std::size_t offset = 0;
      while (offset < section_size) {
        ZydisDecodedInstruction instruction{};
        ZydisDecodedOperand operands[ZYDIS_MAX_OPERAND_COUNT]{};
        if (!ZYAN_SUCCESS(ZydisDecoderDecodeFull(
                &decoder, section_begin + offset, section_size - offset,
                &instruction, operands))) {
          ++offset;
          continue;
        }
        const auto call_address = reinterpret_cast<std::uintptr_t>(section_begin + offset);
        if (instruction.mnemonic == ZYDIS_MNEMONIC_CALL &&
            instruction.operand_count_visible > 0 &&
            operands[0].type == ZYDIS_OPERAND_TYPE_IMMEDIATE &&
            operands[0].imm.is_relative) {
          ZyanU64 absolute = 0;
          if (ZYAN_SUCCESS(ZydisCalcAbsoluteAddress(
                  &instruction, &operands[0], call_address, &absolute)) &&
              absolute == target) {
            DWORD64 lookup_base = static_cast<DWORD64>(image_base);
            const auto entry = RtlLookupFunctionEntry(
                static_cast<DWORD64>(call_address), &lookup_base, nullptr);
            if (entry && lookup_base == image_base &&
                entry->BeginAddress < nt->OptionalHeader.SizeOfImage)
              ++callers[entry->BeginAddress];
          }
        }
        offset += instruction.length;
      }
    }
    for (const auto& [caller_rva, call_sites] : callers) {
      if (result.size() >= 128) return result;
      if (image_base + caller_rva + 32 > image_end) continue;
      result.push_back(
          {item.candidate_id, caller_rva, call_sites,
           Sha256Bytes(std::span(base + caller_rva, std::size_t{32}))});
      if (item.depth < 7 && visited.insert(caller_rva).second)
        work.push_back({"caller_rva_" + std::to_string(caller_rva), caller_rva,
                        item.depth + 1});
    }
  }
  return result;
}

std::vector<StateWriterCandidate> InspectStateWriters(void* image,
                                                      std::uint32_t state_rva) {
  std::vector<StateWriterCandidate> result;
  const std::uint8_t* base = nullptr;
  const IMAGE_NT_HEADERS64* nt = nullptr;
  if (!ReadImage(image, base, nt) || state_rva >= nt->OptionalHeader.SizeOfImage)
    return result;

  const auto image_base = reinterpret_cast<std::uintptr_t>(base);
  const auto target = image_base + state_rva;
  const auto first = IMAGE_FIRST_SECTION(nt);
  ZydisDecoder decoder;
  if (!ZYAN_SUCCESS(ZydisDecoderInit(&decoder, ZYDIS_MACHINE_MODE_LONG_64,
                                      ZYDIS_STACK_WIDTH_64)))
    return result;

  std::set<std::uint32_t> seen_instructions;
  const auto add_writer = [&](std::uint32_t instruction_rva,
                              std::uint16_t write_width) {
    if (!seen_instructions.insert(instruction_rva).second ||
        result.size() >= 256)
      return;
    const auto instruction_address = image_base + instruction_rva;
    DWORD64 lookup_base = static_cast<DWORD64>(image_base);
    const auto entry = RtlLookupFunctionEntry(
        static_cast<DWORD64>(instruction_address), &lookup_base, nullptr);
    const auto function_rva =
        entry != nullptr && lookup_base == image_base &&
                entry->BeginAddress < nt->OptionalHeader.SizeOfImage
            ? entry->BeginAddress
            : instruction_rva;
    if (function_rva > nt->OptionalHeader.SizeOfImage - 32) return;
    result.push_back({"writer_rva_" + std::to_string(function_rva),
                      state_rva,
                      instruction_rva,
                      function_rva,
                      write_width,
                      "unobserved",
                      0,
                      Sha256Bytes(std::span(base + function_rva,
                                            std::size_t{32}))});
  };
  for (unsigned index = 0;
       index < nt->FileHeader.NumberOfSections && result.size() < 256; ++index) {
    const auto& section = first[index];
    if ((section.Characteristics & IMAGE_SCN_MEM_EXECUTE) == 0) continue;
    const auto section_size = static_cast<std::size_t>(section.Misc.VirtualSize);
    const auto section_begin = base + section.VirtualAddress;
    std::size_t offset = 0;
    std::uintptr_t region_end = 0;
    bool region_readable = false;
    std::map<ZydisRegister, std::uintptr_t> register_values;
    while (offset < section_size && result.size() < 256) {
      const auto address = reinterpret_cast<std::uintptr_t>(section_begin + offset);
      if (address >= region_end) {
        MEMORY_BASIC_INFORMATION memory{};
        if (VirtualQuery(section_begin + offset, &memory, sizeof(memory)) == 0)
          break;
        region_end = std::min(
            reinterpret_cast<std::uintptr_t>(section_begin) + section_size,
            reinterpret_cast<std::uintptr_t>(memory.BaseAddress) +
                memory.RegionSize);
        region_readable =
            memory.State == MEM_COMMIT &&
            (memory.Protect & (PAGE_GUARD | PAGE_NOACCESS)) == 0;
      }
      if (!region_readable || region_end <= address) {
        offset += std::max<std::size_t>(1, region_end > address
                                              ? region_end - address
                                              : 1);
        continue;
      }
      const auto available = static_cast<std::size_t>(region_end - address);
      ZydisDecodedInstruction instruction{};
      ZydisDecodedOperand operands[ZYDIS_MAX_OPERAND_COUNT]{};
      if (!ZYAN_SUCCESS(ZydisDecoderDecodeFull(
              &decoder, section_begin + offset, available, &instruction,
              operands))) {
        ++offset;
        continue;
      }
      const auto instruction_address = image_base + section.VirtualAddress + offset;
      const auto instruction_rva =
          section.VirtualAddress + static_cast<std::uint32_t>(offset);
      for (std::uint8_t operand_index = 0;
           operand_index < instruction.operand_count_visible; ++operand_index) {
        const auto& operand = operands[operand_index];
        if (operand.type != ZYDIS_OPERAND_TYPE_MEMORY ||
            operand.mem.base != ZYDIS_REGISTER_RIP ||
            (operand.actions & ZYDIS_OPERAND_ACTION_WRITE) == 0)
          continue;
        ZyanU64 absolute = 0;
        if (!ZYAN_SUCCESS(ZydisCalcAbsoluteAddress(
                &instruction, &operand, instruction_address, &absolute)) ||
            absolute != target)
          continue;
        add_writer(instruction_rva,
                   static_cast<std::uint16_t>(operand.size / 8));
      }

      // Resolve a bounded, intra-basic-block pointer chain. This recognizes
      // code such as `mov rax, [rip+global]; mov [rax+offset], ecx` without
      // reading heaps, stacks or third-party modules.
      for (std::uint8_t operand_index = 0;
           operand_index < instruction.operand_count_visible; ++operand_index) {
        const auto& operand = operands[operand_index];
        if (operand.type != ZYDIS_OPERAND_TYPE_MEMORY ||
            operand.mem.base == ZYDIS_REGISTER_NONE ||
            operand.mem.base == ZYDIS_REGISTER_RIP ||
            operand.mem.index != ZYDIS_REGISTER_NONE ||
            (operand.actions & ZYDIS_OPERAND_ACTION_WRITE) == 0)
          continue;
        const auto base_register = ZydisRegisterGetLargestEnclosing(
            ZYDIS_MACHINE_MODE_LONG_64, operand.mem.base);
        const auto known = register_values.find(base_register);
        if (known == register_values.end()) continue;
        const auto effective = static_cast<std::uintptr_t>(
            static_cast<std::intptr_t>(known->second) + operand.mem.disp.value);
        if (effective == target)
          add_writer(instruction_rva,
                     static_cast<std::uint16_t>(operand.size / 8));
      }

      std::optional<std::pair<ZydisRegister, std::uintptr_t>> propagated;
      if (instruction.operand_count_visible >= 2 &&
          operands[0].type == ZYDIS_OPERAND_TYPE_REGISTER &&
          (operands[0].actions & ZYDIS_OPERAND_ACTION_WRITE) != 0) {
        const auto destination = ZydisRegisterGetLargestEnclosing(
            ZYDIS_MACHINE_MODE_LONG_64, operands[0].reg.value);
        const auto& source = operands[1];
        if (instruction.mnemonic == ZYDIS_MNEMONIC_LEA &&
            source.type == ZYDIS_OPERAND_TYPE_MEMORY &&
            source.mem.base == ZYDIS_REGISTER_RIP) {
          ZyanU64 absolute = 0;
          if (ZYAN_SUCCESS(ZydisCalcAbsoluteAddress(
                  &instruction, &source, instruction_address, &absolute)))
            propagated = std::pair(destination,
                                   static_cast<std::uintptr_t>(absolute));
        } else if (instruction.mnemonic == ZYDIS_MNEMONIC_MOV &&
                   source.type == ZYDIS_OPERAND_TYPE_REGISTER) {
          const auto source_register = ZydisRegisterGetLargestEnclosing(
              ZYDIS_MACHINE_MODE_LONG_64, source.reg.value);
          const auto known = register_values.find(source_register);
          if (known != register_values.end())
            propagated = std::pair(destination, known->second);
        } else if (instruction.mnemonic == ZYDIS_MNEMONIC_MOV &&
                   operands[0].size == 64 &&
                   source.type == ZYDIS_OPERAND_TYPE_MEMORY &&
                   source.mem.base == ZYDIS_REGISTER_RIP) {
          ZyanU64 source_address = 0;
          if (ZYAN_SUCCESS(ZydisCalcAbsoluteAddress(
                  &instruction, &source, instruction_address,
                  &source_address)) &&
              source_address >= image_base &&
              source_address + sizeof(std::uintptr_t) <=
                  image_base + nt->OptionalHeader.SizeOfImage) {
            std::uintptr_t pointer = 0;
            std::memcpy(&pointer,
                        reinterpret_cast<const void*>(source_address),
                        sizeof(pointer));
            if (pointer >= image_base &&
                pointer < image_base + nt->OptionalHeader.SizeOfImage)
              propagated = std::pair(destination, pointer);
          }
        }
      }
      for (std::uint8_t operand_index = 0;
           operand_index < instruction.operand_count_visible; ++operand_index) {
        const auto& operand = operands[operand_index];
        if (operand.type == ZYDIS_OPERAND_TYPE_REGISTER &&
            (operand.actions & ZYDIS_OPERAND_ACTION_WRITE) != 0)
          register_values.erase(ZydisRegisterGetLargestEnclosing(
              ZYDIS_MACHINE_MODE_LONG_64, operand.reg.value));
      }
      if (propagated) register_values.insert_or_assign(propagated->first,
                                                       propagated->second);
      if (instruction.meta.category == ZYDIS_CATEGORY_CALL ||
          instruction.meta.category == ZYDIS_CATEGORY_RET ||
          instruction.meta.category == ZYDIS_CATEGORY_COND_BR ||
          instruction.meta.category == ZYDIS_CATEGORY_UNCOND_BR)
        register_values.clear();
      offset += instruction.length;
    }
  }
  return result;
}

}  // namespace gameverse
