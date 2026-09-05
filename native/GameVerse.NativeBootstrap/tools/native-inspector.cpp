#include "gameverse/bootstrap.hpp"

#include <windows.h>
#include <Zydis/Zydis.h>

#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <iterator>
#include <optional>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

namespace {

struct Section {
  std::string name;
  std::uint32_t rva{};
  std::uint32_t virtual_size{};
  std::uint32_t raw_offset{};
  std::uint32_t raw_size{};
  std::uint32_t characteristics{};
};

struct Image {
  std::vector<std::uint8_t> bytes;
  std::vector<Section> sections;

  std::optional<std::size_t> FileOffset(std::uint32_t rva) const {
    for (const auto& section : sections) {
      const auto span = std::max(section.virtual_size, section.raw_size);
      if (rva >= section.rva && rva - section.rva < span) {
        const auto offset = static_cast<std::size_t>(section.raw_offset) + rva - section.rva;
        if (offset < bytes.size()) return offset;
      }
    }
    return std::nullopt;
  }
};

Image LoadImage(const std::filesystem::path& path) {
  std::ifstream input(path, std::ios::binary);
  if (!input) throw std::runtime_error("image_unavailable");
  std::vector<std::uint8_t> bytes((std::istreambuf_iterator<char>(input)),
                                  std::istreambuf_iterator<char>());
  Image image{std::move(bytes), {}};
  if (image.bytes.size() < sizeof(IMAGE_DOS_HEADER))
    throw std::runtime_error("invalid_pe_image");
  const auto* dos = reinterpret_cast<const IMAGE_DOS_HEADER*>(image.bytes.data());
  if (dos->e_magic != IMAGE_DOS_SIGNATURE || dos->e_lfanew <= 0 ||
      static_cast<std::size_t>(dos->e_lfanew) + sizeof(IMAGE_NT_HEADERS64) > image.bytes.size())
    throw std::runtime_error("invalid_pe_image");
  const auto* nt = reinterpret_cast<const IMAGE_NT_HEADERS64*>(image.bytes.data() + dos->e_lfanew);
  if (nt->Signature != IMAGE_NT_SIGNATURE ||
      nt->OptionalHeader.Magic != IMAGE_NT_OPTIONAL_HDR64_MAGIC)
    throw std::runtime_error("invalid_pe_image");
  const auto* first = IMAGE_FIRST_SECTION(nt);
  for (unsigned index = 0; index < nt->FileHeader.NumberOfSections; ++index) {
    const auto& source = first[index];
    char name[IMAGE_SIZEOF_SHORT_NAME + 1]{};
    std::copy_n(reinterpret_cast<const char*>(source.Name), IMAGE_SIZEOF_SHORT_NAME, name);
    image.sections.push_back(Section{name, source.VirtualAddress, source.Misc.VirtualSize,
                                     source.PointerToRawData, source.SizeOfRawData,
                                     source.Characteristics});
  }
  return image;
}

std::string Narrow(std::wstring_view value) {
  if (value.empty()) return {};
  const int size = WideCharToMultiByte(CP_UTF8, WC_ERR_INVALID_CHARS, value.data(),
                                       static_cast<int>(value.size()), nullptr, 0,
                                       nullptr, nullptr);
  if (size <= 0) throw std::runtime_error("invalid_utf16_argument");
  std::string result(static_cast<std::size_t>(size), '\0');
  WideCharToMultiByte(CP_UTF8, WC_ERR_INVALID_CHARS, value.data(),
                      static_cast<int>(value.size()), result.data(), size, nullptr,
                      nullptr);
  return result;
}

std::uint32_t ParseRva(std::wstring_view value) {
  std::size_t used = 0;
  const auto parsed = std::stoull(std::wstring(value), &used, 0);
  if (used != value.size() || parsed > 0xffff'ffffULL)
    throw std::runtime_error("invalid_rva");
  return static_cast<std::uint32_t>(parsed);
}

std::string Hex(std::uint64_t value) {
  std::ostringstream output;
  output << "0x" << std::uppercase << std::hex << value;
  return output.str();
}

std::string PatternText(const std::vector<gameverse::PatternByte>& pattern) {
  std::ostringstream output;
  output << std::uppercase << std::hex << std::setfill('0');
  for (std::size_t index = 0; index < pattern.size(); ++index) {
    if (index != 0) output << ' ';
    if (pattern[index].wildcard)
      output << "??";
    else
      output << std::setw(2) << static_cast<unsigned>(pattern[index].value);
  }
  return output.str();
}

std::vector<gameverse::PatternByte> BuildPattern(const Image& image, std::uint32_t rva,
                                                  std::size_t requested) {
  const auto file_offset = image.FileOffset(rva);
  if (!file_offset) throw std::runtime_error("candidate_outside_image");
  const auto available = image.bytes.size() - *file_offset;
  const auto maximum = std::min<std::size_t>({available, requested, 64});
  ZydisDecoder decoder;
  if (!ZYAN_SUCCESS(ZydisDecoderInit(&decoder, ZYDIS_MACHINE_MODE_LONG_64,
                                      ZYDIS_STACK_WIDTH_64)))
    throw std::runtime_error("decoder_initialization_failed");
  std::vector<gameverse::PatternByte> pattern;
  std::size_t offset = 0;
  while (offset < maximum) {
    ZydisDecodedInstruction instruction{};
    ZydisDecodedOperand operands[ZYDIS_MAX_OPERAND_COUNT]{};
    if (!ZYAN_SUCCESS(ZydisDecoderDecodeFull(&decoder, image.bytes.data() + *file_offset + offset,
                                             maximum - offset, &instruction, operands)))
      break;
    const auto begin = pattern.size();
    for (std::size_t byte = 0; byte < instruction.length; ++byte)
      pattern.push_back({image.bytes[*file_offset + offset + byte], false});
    bool rip_relative = false;
    for (std::uint8_t operand_index = 0; operand_index < instruction.operand_count_visible;
         ++operand_index) {
      const auto& operand = operands[operand_index];
      rip_relative = rip_relative ||
                     (operand.type == ZYDIS_OPERAND_TYPE_MEMORY &&
                      operand.mem.base == ZYDIS_REGISTER_RIP);
    }
    if (rip_relative && instruction.raw.disp.size != 0) {
      for (std::size_t byte = 0; byte < instruction.raw.disp.size / 8; ++byte)
        pattern[begin + instruction.raw.disp.offset + byte].wildcard = true;
    }
    for (const auto& immediate : instruction.raw.imm) {
      if (!immediate.is_relative || immediate.size == 0) continue;
      for (std::size_t byte = 0; byte < immediate.size / 8; ++byte)
        pattern[begin + immediate.offset + byte].wildcard = true;
    }
    offset += instruction.length;
  }
  if (pattern.size() < 8) throw std::runtime_error("candidate_decode_too_short");
  return pattern;
}

std::size_t UniqueMatches(const Image& image,
                          const std::vector<gameverse::PatternByte>& pattern) {
  std::size_t matches = 0;
  for (const auto& section : image.sections) {
    if ((section.characteristics & IMAGE_SCN_MEM_EXECUTE) == 0 ||
        section.raw_offset >= image.bytes.size())
      continue;
    const auto size = std::min<std::size_t>(section.raw_size,
                                            image.bytes.size() - section.raw_offset);
    matches += gameverse::FindPattern(
                   std::span(image.bytes.data() + section.raw_offset, size), pattern)
                   .size();
  }
  return matches;
}

std::vector<std::uint32_t> FindStringRvas(const Image& image, const std::string& value) {
  if (value.empty() || value.size() > 256) throw std::runtime_error("invalid_search_string");
  std::vector<std::uint32_t> output;
  const std::vector<std::uint8_t> needle(value.begin(), value.end());
  for (const auto& section : image.sections) {
    if (section.raw_offset >= image.bytes.size()) continue;
    const auto size = std::min<std::size_t>(section.raw_size,
                                            image.bytes.size() - section.raw_offset);
    const auto begin = image.bytes.begin() + section.raw_offset;
    auto cursor = begin;
    while (cursor != begin + size) {
      cursor = std::search(cursor, begin + size, needle.begin(), needle.end());
      if (cursor == begin + size) break;
      output.push_back(section.rva + static_cast<std::uint32_t>(cursor - begin));
      ++cursor;
      if (output.size() >= 128) return output;
    }
  }
  return output;
}

std::vector<std::uint32_t> FindReferences(
    const Image& image, const std::vector<std::uint32_t>& targets) {
  std::vector<std::uint32_t> references;
  ZydisDecoder decoder;
  if (!ZYAN_SUCCESS(ZydisDecoderInit(&decoder, ZYDIS_MACHINE_MODE_LONG_64,
                                      ZYDIS_STACK_WIDTH_64)))
    throw std::runtime_error("decoder_initialization_failed");
  for (const auto& section : image.sections) {
    if ((section.characteristics & IMAGE_SCN_MEM_EXECUTE) == 0 ||
        section.raw_offset >= image.bytes.size())
      continue;
    const auto size = std::min<std::size_t>(section.raw_size,
                                            image.bytes.size() - section.raw_offset);
    std::size_t offset = 0;
    while (offset < size && references.size() < 256) {
      ZydisDecodedInstruction instruction{};
      ZydisDecodedOperand operands[ZYDIS_MAX_OPERAND_COUNT]{};
      if (!ZYAN_SUCCESS(ZydisDecoderDecodeFull(
              &decoder, image.bytes.data() + section.raw_offset + offset,
              size - offset, &instruction, operands))) {
        ++offset;
        continue;
      }
      const auto instruction_rva =
          static_cast<std::uint64_t>(section.rva) + offset;
      for (std::uint8_t operand_index = 0;
           operand_index < instruction.operand_count_visible; ++operand_index) {
        const auto& operand = operands[operand_index];
        if (!((operand.type == ZYDIS_OPERAND_TYPE_MEMORY &&
               operand.mem.base == ZYDIS_REGISTER_RIP) ||
              (operand.type == ZYDIS_OPERAND_TYPE_IMMEDIATE &&
               operand.imm.is_relative)))
          continue;
        ZyanU64 absolute = 0;
        if (ZYAN_SUCCESS(ZydisCalcAbsoluteAddress(
                &instruction, &operand, instruction_rva, &absolute)) &&
            absolute <= 0xffff'ffffULL &&
            std::binary_search(targets.begin(), targets.end(),
                               static_cast<std::uint32_t>(absolute))) {
          references.push_back(static_cast<std::uint32_t>(instruction_rva));
          break;
        }
      }
      offset += instruction.length;
    }
  }
  std::sort(references.begin(), references.end());
  references.erase(std::unique(references.begin(), references.end()), references.end());
  return references;
}

}  // namespace

int wmain(int argc, wchar_t** argv) {
  try {
    std::filesystem::path image_path;
    std::optional<std::uint32_t> candidate;
    std::optional<std::string> search;
    std::size_t length = 32;
    for (int index = 1; index < argc; ++index) {
      const std::wstring_view argument(argv[index]);
      if (argument == L"--image" && index + 1 < argc)
        image_path = argv[++index];
      else if (argument == L"--candidate-rva" && index + 1 < argc)
        candidate = ParseRva(argv[++index]);
      else if (argument == L"--length" && index + 1 < argc)
        length = static_cast<std::size_t>(ParseRva(argv[++index]));
      else if (argument == L"--string" && index + 1 < argc) {
        const std::wstring value(argv[++index]);
        search = Narrow(value);
      } else
        throw std::runtime_error("unsupported_argument");
    }
    if (image_path.empty() || (!candidate && !search))
      throw std::runtime_error("usage: --image PATH (--candidate-rva RVA | --string TEXT)");
    const auto image = LoadImage(image_path);
    std::cout << "{\"schema_version\":1,\"image_sha256\":\""
              << gameverse::Sha256File(image_path) << "\"";
    if (candidate) {
      const auto pattern = BuildPattern(image, *candidate, length);
      const auto matches = UniqueMatches(image, pattern);
      const auto first_wildcard = std::find_if(pattern.begin(), pattern.end(),
                                               [](const auto& value) { return value.wildcard; });
      const auto stable_prefix = static_cast<std::size_t>(first_wildcard - pattern.begin());
      if (stable_prefix < 2) throw std::runtime_error("candidate_has_no_stable_prologue");
      std::vector<gameverse::PatternByte> prologue(
          pattern.begin(), pattern.begin() + std::min<std::size_t>(stable_prefix, 8));
      std::cout << ",\"candidate\":{\"rva\":\"" << Hex(*candidate)
                << "\",\"pattern\":\"" << PatternText(pattern)
                << "\",\"prologue\":\"" << PatternText(prologue)
                << "\",\"unique_match_count\":" << matches << '}';
    }
    if (search) {
      auto rvas = FindStringRvas(image, *search);
      std::sort(rvas.begin(), rvas.end());
      const auto references = FindReferences(image, rvas);
      std::cout << ",\"string_query\":{\"sha256\":\""
                << gameverse::Sha256Bytes(std::span(
                       reinterpret_cast<const std::uint8_t*>(search->data()), search->size()))
                << "\",\"matches\":[";
      for (std::size_t index = 0; index < rvas.size(); ++index) {
        if (index != 0) std::cout << ',';
        std::cout << '"' << Hex(rvas[index]) << '"';
      }
      std::cout << "],\"reference_rvas\":[";
      for (std::size_t index = 0; index < references.size(); ++index) {
        if (index != 0) std::cout << ',';
        std::cout << '"' << Hex(references[index]) << '"';
      }
      std::cout << "]}";
    }
    std::cout << "}\n";
    return 0;
  } catch (const std::exception& error) {
    std::cerr << "{\"schema_version\":1,\"error\":\""
              << gameverse::JsonEscape(error.what()) << "\"}\n";
    return 1;
  }
}
