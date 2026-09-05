#include "gameverse/bootstrap.hpp"

#include <windows.h>
#include <intrin.h>

#include <algorithm>
#include <cstring>
#include <set>

namespace gameverse {
namespace {

constexpr std::uint64_t kWindowMilliseconds = 20'000;
constexpr std::size_t kPageBytes = 4096;
constexpr std::size_t kMaximumPages = 16'384;
constexpr std::size_t kMaximumTransitions = 32;
// The bootstrap sends these in bounded batches. Keeping a wider local set is
// required because a Story transition can change thousands of small scalars;
// truncating before comparison can discard the actual init-state.
constexpr std::size_t kMaximumResults = 8192;

std::string SectionName(const IMAGE_SECTION_HEADER& section) {
  char name[IMAGE_SIZEOF_SHORT_NAME + 1]{};
  std::memcpy(name, section.Name, IMAGE_SIZEOF_SHORT_NAME);
  return name;
}

bool ReadableWritableImagePage(const void* address, const void* image) {
  MEMORY_BASIC_INFORMATION memory{};
  if (VirtualQuery(address, &memory, sizeof(memory)) != sizeof(memory) ||
      memory.State != MEM_COMMIT || memory.Type != MEM_IMAGE ||
      memory.AllocationBase != image || (memory.Protect & PAGE_GUARD) != 0 ||
      (memory.Protect & PAGE_NOACCESS) != 0)
    return false;
  const auto protection = memory.Protect & 0xff;
  // Some ASI loaders leave the whole mapped image executable-writable even
  // though the PE section itself is not executable. Section characteristics
  // are checked separately; accept these page protections so the supported
  // image can still be observed without changing them.
  return protection == PAGE_READWRITE || protection == PAGE_WRITECOPY ||
         protection == PAGE_EXECUTE_READWRITE ||
         protection == PAGE_EXECUTE_WRITECOPY;
}

std::uint64_t FastDigest(std::span<const std::uint8_t> bytes) {
  std::uint64_t first = 0x243F6A88U;
  std::uint64_t second = 0x85A308D3U;
  std::size_t offset = 0;
  while (offset + sizeof(std::uint64_t) <= bytes.size()) {
    std::uint64_t word = 0;
    std::memcpy(&word, bytes.data() + offset, sizeof(word));
    first = _mm_crc32_u64(first, word);
    second = _mm_crc32_u64(second, word ^ 0x9E3779B97F4A7C15ULL);
    offset += sizeof(word);
  }
  while (offset < bytes.size()) {
    first = _mm_crc32_u8(static_cast<std::uint32_t>(first), bytes[offset]);
    second = _mm_crc32_u8(static_cast<std::uint32_t>(second),
                          static_cast<std::uint8_t>(bytes[offset] ^ 0xA5));
    ++offset;
  }
  return (first << 32) | second;
}

bool IsSmallState(std::uint32_t value) { return value <= 64; }

}  // namespace

InitStateSampler::InitStateSampler(void* image) : image_(image) {}

bool InitStateSampler::Start(std::string_view marker_id) noexcept {
  active_ = false;
  pages_.clear();
  sequences_.clear();
  marker_id_ = std::string(marker_id.substr(0, 64));
  if (!image_ || marker_id_.empty()) return false;
  try {
    int processor[4]{};
    __cpuid(processor, 1);
    if ((processor[2] & (1 << 20)) == 0) return false;
    const auto* base = static_cast<const std::uint8_t*>(image_);
    const auto* dos = reinterpret_cast<const IMAGE_DOS_HEADER*>(base);
    if (dos->e_magic != IMAGE_DOS_SIGNATURE || dos->e_lfanew <= 0) return false;
    const auto* nt = reinterpret_cast<const IMAGE_NT_HEADERS64*>(base + dos->e_lfanew);
    if (nt->Signature != IMAGE_NT_SIGNATURE ||
        nt->OptionalHeader.Magic != IMAGE_NT_OPTIONAL_HDR64_MAGIC)
      return false;
    const auto* sections = IMAGE_FIRST_SECTION(nt);
    for (unsigned index = 0;
         index < nt->FileHeader.NumberOfSections && pages_.size() < kMaximumPages;
         ++index) {
      const auto& section = sections[index];
      if ((section.Characteristics & IMAGE_SCN_MEM_WRITE) == 0 ||
          (section.Characteristics & IMAGE_SCN_MEM_EXECUTE) != 0)
        continue;
      const auto section_name = SectionName(section);
      const auto section_size = static_cast<std::size_t>(section.Misc.VirtualSize);
      for (std::size_t offset = 0;
           offset < section_size && pages_.size() < kMaximumPages;
           offset += kPageBytes) {
        const auto length = std::min(kPageBytes, section_size - offset);
        const auto* address = base + section.VirtualAddress + offset;
        if (length < sizeof(std::uint32_t) ||
            !ReadableWritableImagePage(address, image_))
          continue;
        Page page;
        page.address = address;
        page.rva = section.VirtualAddress + static_cast<std::uint32_t>(offset);
        page.section = section_name;
        page.values.resize(length / sizeof(std::uint32_t));
        std::memcpy(page.values.data(), address,
                    page.values.size() * sizeof(std::uint32_t));
        page.digest = FastDigest(std::span(address, length));
        pages_.push_back(std::move(page));
      }
    }
    deadline_ms_ = MonotonicMilliseconds() + kWindowMilliseconds;
    active_ = !pages_.empty();
    return active_;
  } catch (...) {
    pages_.clear();
    sequences_.clear();
    return false;
  }
}

bool InitStateSampler::Poll() noexcept {
  if (!active_) return false;
  if (expired()) return false;
  try {
    for (auto& page : pages_) {
      const auto length = page.values.size() * sizeof(std::uint32_t);
      if (!ReadableWritableImagePage(page.address, image_)) continue;
      const auto digest = FastDigest(std::span(page.address, length));
      if (digest == page.digest) continue;
      std::vector<std::uint32_t> current(page.values.size());
      std::memcpy(current.data(), page.address, length);
      for (std::size_t index = 0; index < current.size(); ++index) {
        if (current[index] == page.values[index] ||
            (!IsSmallState(current[index]) && !IsSmallState(page.values[index])))
          continue;
        const auto rva = page.rva + static_cast<std::uint32_t>(index * 4);
        auto& sequence = sequences_[rva];
        if (sequence.values.empty()) {
          sequence.section = page.section;
          sequence.values.push_back(page.values[index]);
          sequence.distinct.insert(page.values[index]);
        }
        if (sequence.values.size() <= kMaximumTransitions &&
            sequence.values.back() != current[index]) {
          sequence.values.push_back(current[index]);
          sequence.distinct.insert(current[index]);
        }
      }
      page.values = std::move(current);
      page.digest = digest;
    }
    return true;
  } catch (...) {
    active_ = false;
    return false;
  }
}

bool InitStateSampler::expired() const noexcept {
  return active_ && MonotonicMilliseconds() >= deadline_ms_;
}

std::vector<InitStateCandidate> InitStateSampler::Finish() noexcept {
  active_ = false;
  std::vector<InitStateCandidate> result;
  try {
    for (const auto& [rva, sequence] : sequences_) {
      if (sequence.values.size() < 2 ||
          sequence.values.size() > kMaximumTransitions + 1 ||
          sequence.distinct.size() < 2 || sequence.distinct.size() > 16)
        continue;
      const auto bytes = std::span(
          reinterpret_cast<const std::uint8_t*>(sequence.values.data()),
          sequence.values.size() * sizeof(std::uint32_t));
      result.push_back({"state_rva_" + std::to_string(rva), rva,
                        sequence.section,
                        static_cast<std::uint16_t>(sequence.values.size() - 1),
                        static_cast<std::uint16_t>(sequence.distinct.size()),
                        Sha256Bytes(bytes), marker_id_});
    }
    std::sort(result.begin(), result.end(), [](const auto& left, const auto& right) {
      if (left.distinct_state_count != right.distinct_state_count)
        return left.distinct_state_count < right.distinct_state_count;
      if (left.transition_count != right.transition_count)
        return left.transition_count < right.transition_count;
      return left.rva < right.rva;
    });
    if (result.size() > kMaximumResults) result.resize(kMaximumResults);
  } catch (...) {
    result.clear();
  }
  pages_.clear();
  sequences_.clear();
  return result;
}

std::string SerializeTelemetryMarker(std::string_view marker_id,
                                     std::uint64_t monotonic_ms) {
  return "{\"type\":\"telemetry_marker_v1\",\"schema_version\":1,\"marker_id\":\"" +
         JsonEscape(marker_id) + "\",\"monotonic_ms\":" +
         std::to_string(monotonic_ms) + "}";
}

std::string SerializeInitStateCandidates(
    std::span<const InitStateCandidate> candidates) {
  std::string output =
      "{\"type\":\"init_state_candidates_v1\",\"schema_version\":1,\"candidates\":[";
  for (std::size_t index = 0; index < candidates.size(); ++index) {
    if (index != 0) output += ',';
    const auto& candidate = candidates[index];
    output += "{\"candidate_id\":\"" + JsonEscape(candidate.candidate_id) +
              "\",\"rva\":" + std::to_string(candidate.rva) +
              ",\"section\":\"" + JsonEscape(candidate.section) +
              "\",\"transition_count\":" +
              std::to_string(candidate.transition_count) +
              ",\"distinct_state_count\":" +
              std::to_string(candidate.distinct_state_count) +
              ",\"sequence_sha256\":\"" + candidate.sequence_sha256 +
              "\",\"stage_correlation\":\"" +
              JsonEscape(candidate.stage_correlation) + "\"}";
  }
  output += "]}";
  return output;
}

}  // namespace gameverse
