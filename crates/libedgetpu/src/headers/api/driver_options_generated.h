// automatically generated by the FlatBuffers compiler, do not modify


#ifndef FLATBUFFERS_GENERATED_DRIVEROPTIONS_PLATFORMS_DARWINN_API_H_
#define FLATBUFFERS_GENERATED_DRIVEROPTIONS_PLATFORMS_DARWINN_API_H_

#include "flatbuffers/flatbuffers.h"

// Ensure the included flatbuffers.h is the same version as when this file was
// generated, otherwise it may not be compatible.
static_assert(FLATBUFFERS_VERSION_MAJOR == 24 &&
              FLATBUFFERS_VERSION_MINOR == 3 &&
              FLATBUFFERS_VERSION_REVISION == 25,
             "Non-compatible flatbuffers version included");

namespace platforms {
namespace darwinn {
namespace api {

struct DriverUsbOptions;
struct DriverUsbOptionsBuilder;

struct DriverOptions;
struct DriverOptionsBuilder;

enum PerformanceExpectation : int32_t {
  PerformanceExpectation_Low = 0,
  PerformanceExpectation_Medium = 1,
  PerformanceExpectation_High = 2,
  PerformanceExpectation_Max = 3,
  PerformanceExpectation_MIN = PerformanceExpectation_Low,
  PerformanceExpectation_MAX = PerformanceExpectation_Max
};

inline const PerformanceExpectation (&EnumValuesPerformanceExpectation())[4] {
  static const PerformanceExpectation values[] = {
    PerformanceExpectation_Low,
    PerformanceExpectation_Medium,
    PerformanceExpectation_High,
    PerformanceExpectation_Max
  };
  return values;
}

inline const char * const *EnumNamesPerformanceExpectation() {
  static const char * const names[5] = {
    "Low",
    "Medium",
    "High",
    "Max",
    nullptr
  };
  return names;
}

inline const char *EnumNamePerformanceExpectation(PerformanceExpectation e) {
  if (::flatbuffers::IsOutRange(e, PerformanceExpectation_Low, PerformanceExpectation_Max)) return "";
  const size_t index = static_cast<size_t>(e);
  return EnumNamesPerformanceExpectation()[index];
}

struct DriverUsbOptions FLATBUFFERS_FINAL_CLASS : private ::flatbuffers::Table {
  typedef DriverUsbOptionsBuilder Builder;
  enum FlatBuffersVTableOffset FLATBUFFERS_VTABLE_UNDERLYING_TYPE {
    VT_DFU_FIRMWARE = 4,
    VT_ALWAYS_DFU = 6,
    VT_HAS_FAIL_IF_SLOWER_THAN_SUPERSPEED = 8,
    VT_FAIL_IF_SLOWER_THAN_SUPERSPEED = 10,
    VT_HAS_FORCE_LARGEST_BULK_IN_CHUNK_SIZE = 12,
    VT_FORCE_LARGEST_BULK_IN_CHUNK_SIZE = 14,
    VT_HAS_ENABLE_OVERLAPPING_BULK_IN_AND_OUT = 16,
    VT_ENABLE_OVERLAPPING_BULK_IN_AND_OUT = 18,
    VT_HAS_ENABLE_QUEUED_BULK_IN_REQUESTS = 20,
    VT_ENABLE_QUEUED_BULK_IN_REQUESTS = 22,
    VT_HAS_BULK_IN_QUEUE_CAPACITY = 24,
    VT_BULK_IN_QUEUE_CAPACITY = 26
  };
  const ::flatbuffers::String *dfu_firmware() const {
    return GetPointer<const ::flatbuffers::String *>(VT_DFU_FIRMWARE);
  }
  bool always_dfu() const {
    return GetField<uint8_t>(VT_ALWAYS_DFU, 1) != 0;
  }
  bool has_fail_if_slower_than_superspeed() const {
    return GetField<uint8_t>(VT_HAS_FAIL_IF_SLOWER_THAN_SUPERSPEED, 0) != 0;
  }
  bool fail_if_slower_than_superspeed() const {
    return GetField<uint8_t>(VT_FAIL_IF_SLOWER_THAN_SUPERSPEED, 0) != 0;
  }
  bool has_force_largest_bulk_in_chunk_size() const {
    return GetField<uint8_t>(VT_HAS_FORCE_LARGEST_BULK_IN_CHUNK_SIZE, 0) != 0;
  }
  bool force_largest_bulk_in_chunk_size() const {
    return GetField<uint8_t>(VT_FORCE_LARGEST_BULK_IN_CHUNK_SIZE, 0) != 0;
  }
  bool has_enable_overlapping_bulk_in_and_out() const {
    return GetField<uint8_t>(VT_HAS_ENABLE_OVERLAPPING_BULK_IN_AND_OUT, 0) != 0;
  }
  bool enable_overlapping_bulk_in_and_out() const {
    return GetField<uint8_t>(VT_ENABLE_OVERLAPPING_BULK_IN_AND_OUT, 1) != 0;
  }
  bool has_enable_queued_bulk_in_requests() const {
    return GetField<uint8_t>(VT_HAS_ENABLE_QUEUED_BULK_IN_REQUESTS, 0) != 0;
  }
  bool enable_queued_bulk_in_requests() const {
    return GetField<uint8_t>(VT_ENABLE_QUEUED_BULK_IN_REQUESTS, 1) != 0;
  }
  bool has_bulk_in_queue_capacity() const {
    return GetField<uint8_t>(VT_HAS_BULK_IN_QUEUE_CAPACITY, 0) != 0;
  }
  int32_t bulk_in_queue_capacity() const {
    return GetField<int32_t>(VT_BULK_IN_QUEUE_CAPACITY, 32);
  }
  bool Verify(::flatbuffers::Verifier &verifier) const {
    return VerifyTableStart(verifier) &&
           VerifyOffset(verifier, VT_DFU_FIRMWARE) &&
           verifier.VerifyString(dfu_firmware()) &&
           VerifyField<uint8_t>(verifier, VT_ALWAYS_DFU, 1) &&
           VerifyField<uint8_t>(verifier, VT_HAS_FAIL_IF_SLOWER_THAN_SUPERSPEED, 1) &&
           VerifyField<uint8_t>(verifier, VT_FAIL_IF_SLOWER_THAN_SUPERSPEED, 1) &&
           VerifyField<uint8_t>(verifier, VT_HAS_FORCE_LARGEST_BULK_IN_CHUNK_SIZE, 1) &&
           VerifyField<uint8_t>(verifier, VT_FORCE_LARGEST_BULK_IN_CHUNK_SIZE, 1) &&
           VerifyField<uint8_t>(verifier, VT_HAS_ENABLE_OVERLAPPING_BULK_IN_AND_OUT, 1) &&
           VerifyField<uint8_t>(verifier, VT_ENABLE_OVERLAPPING_BULK_IN_AND_OUT, 1) &&
           VerifyField<uint8_t>(verifier, VT_HAS_ENABLE_QUEUED_BULK_IN_REQUESTS, 1) &&
           VerifyField<uint8_t>(verifier, VT_ENABLE_QUEUED_BULK_IN_REQUESTS, 1) &&
           VerifyField<uint8_t>(verifier, VT_HAS_BULK_IN_QUEUE_CAPACITY, 1) &&
           VerifyField<int32_t>(verifier, VT_BULK_IN_QUEUE_CAPACITY, 4) &&
           verifier.EndTable();
  }
};

struct DriverUsbOptionsBuilder {
  typedef DriverUsbOptions Table;
  ::flatbuffers::FlatBufferBuilder &fbb_;
  ::flatbuffers::uoffset_t start_;
  void add_dfu_firmware(::flatbuffers::Offset<::flatbuffers::String> dfu_firmware) {
    fbb_.AddOffset(DriverUsbOptions::VT_DFU_FIRMWARE, dfu_firmware);
  }
  void add_always_dfu(bool always_dfu) {
    fbb_.AddElement<uint8_t>(DriverUsbOptions::VT_ALWAYS_DFU, static_cast<uint8_t>(always_dfu), 1);
  }
  void add_has_fail_if_slower_than_superspeed(bool has_fail_if_slower_than_superspeed) {
    fbb_.AddElement<uint8_t>(DriverUsbOptions::VT_HAS_FAIL_IF_SLOWER_THAN_SUPERSPEED, static_cast<uint8_t>(has_fail_if_slower_than_superspeed), 0);
  }
  void add_fail_if_slower_than_superspeed(bool fail_if_slower_than_superspeed) {
    fbb_.AddElement<uint8_t>(DriverUsbOptions::VT_FAIL_IF_SLOWER_THAN_SUPERSPEED, static_cast<uint8_t>(fail_if_slower_than_superspeed), 0);
  }
  void add_has_force_largest_bulk_in_chunk_size(bool has_force_largest_bulk_in_chunk_size) {
    fbb_.AddElement<uint8_t>(DriverUsbOptions::VT_HAS_FORCE_LARGEST_BULK_IN_CHUNK_SIZE, static_cast<uint8_t>(has_force_largest_bulk_in_chunk_size), 0);
  }
  void add_force_largest_bulk_in_chunk_size(bool force_largest_bulk_in_chunk_size) {
    fbb_.AddElement<uint8_t>(DriverUsbOptions::VT_FORCE_LARGEST_BULK_IN_CHUNK_SIZE, static_cast<uint8_t>(force_largest_bulk_in_chunk_size), 0);
  }
  void add_has_enable_overlapping_bulk_in_and_out(bool has_enable_overlapping_bulk_in_and_out) {
    fbb_.AddElement<uint8_t>(DriverUsbOptions::VT_HAS_ENABLE_OVERLAPPING_BULK_IN_AND_OUT, static_cast<uint8_t>(has_enable_overlapping_bulk_in_and_out), 0);
  }
  void add_enable_overlapping_bulk_in_and_out(bool enable_overlapping_bulk_in_and_out) {
    fbb_.AddElement<uint8_t>(DriverUsbOptions::VT_ENABLE_OVERLAPPING_BULK_IN_AND_OUT, static_cast<uint8_t>(enable_overlapping_bulk_in_and_out), 1);
  }
  void add_has_enable_queued_bulk_in_requests(bool has_enable_queued_bulk_in_requests) {
    fbb_.AddElement<uint8_t>(DriverUsbOptions::VT_HAS_ENABLE_QUEUED_BULK_IN_REQUESTS, static_cast<uint8_t>(has_enable_queued_bulk_in_requests), 0);
  }
  void add_enable_queued_bulk_in_requests(bool enable_queued_bulk_in_requests) {
    fbb_.AddElement<uint8_t>(DriverUsbOptions::VT_ENABLE_QUEUED_BULK_IN_REQUESTS, static_cast<uint8_t>(enable_queued_bulk_in_requests), 1);
  }
  void add_has_bulk_in_queue_capacity(bool has_bulk_in_queue_capacity) {
    fbb_.AddElement<uint8_t>(DriverUsbOptions::VT_HAS_BULK_IN_QUEUE_CAPACITY, static_cast<uint8_t>(has_bulk_in_queue_capacity), 0);
  }
  void add_bulk_in_queue_capacity(int32_t bulk_in_queue_capacity) {
    fbb_.AddElement<int32_t>(DriverUsbOptions::VT_BULK_IN_QUEUE_CAPACITY, bulk_in_queue_capacity, 32);
  }
  explicit DriverUsbOptionsBuilder(::flatbuffers::FlatBufferBuilder &_fbb)
        : fbb_(_fbb) {
    start_ = fbb_.StartTable();
  }
  ::flatbuffers::Offset<DriverUsbOptions> Finish() {
    const auto end = fbb_.EndTable(start_);
    auto o = ::flatbuffers::Offset<DriverUsbOptions>(end);
    return o;
  }
};

inline ::flatbuffers::Offset<DriverUsbOptions> CreateDriverUsbOptions(
    ::flatbuffers::FlatBufferBuilder &_fbb,
    ::flatbuffers::Offset<::flatbuffers::String> dfu_firmware = 0,
    bool always_dfu = true,
    bool has_fail_if_slower_than_superspeed = false,
    bool fail_if_slower_than_superspeed = false,
    bool has_force_largest_bulk_in_chunk_size = false,
    bool force_largest_bulk_in_chunk_size = false,
    bool has_enable_overlapping_bulk_in_and_out = false,
    bool enable_overlapping_bulk_in_and_out = true,
    bool has_enable_queued_bulk_in_requests = false,
    bool enable_queued_bulk_in_requests = true,
    bool has_bulk_in_queue_capacity = false,
    int32_t bulk_in_queue_capacity = 32) {
  DriverUsbOptionsBuilder builder_(_fbb);
  builder_.add_bulk_in_queue_capacity(bulk_in_queue_capacity);
  builder_.add_dfu_firmware(dfu_firmware);
  builder_.add_has_bulk_in_queue_capacity(has_bulk_in_queue_capacity);
  builder_.add_enable_queued_bulk_in_requests(enable_queued_bulk_in_requests);
  builder_.add_has_enable_queued_bulk_in_requests(has_enable_queued_bulk_in_requests);
  builder_.add_enable_overlapping_bulk_in_and_out(enable_overlapping_bulk_in_and_out);
  builder_.add_has_enable_overlapping_bulk_in_and_out(has_enable_overlapping_bulk_in_and_out);
  builder_.add_force_largest_bulk_in_chunk_size(force_largest_bulk_in_chunk_size);
  builder_.add_has_force_largest_bulk_in_chunk_size(has_force_largest_bulk_in_chunk_size);
  builder_.add_fail_if_slower_than_superspeed(fail_if_slower_than_superspeed);
  builder_.add_has_fail_if_slower_than_superspeed(has_fail_if_slower_than_superspeed);
  builder_.add_always_dfu(always_dfu);
  return builder_.Finish();
}

inline ::flatbuffers::Offset<DriverUsbOptions> CreateDriverUsbOptionsDirect(
    ::flatbuffers::FlatBufferBuilder &_fbb,
    const char *dfu_firmware = nullptr,
    bool always_dfu = true,
    bool has_fail_if_slower_than_superspeed = false,
    bool fail_if_slower_than_superspeed = false,
    bool has_force_largest_bulk_in_chunk_size = false,
    bool force_largest_bulk_in_chunk_size = false,
    bool has_enable_overlapping_bulk_in_and_out = false,
    bool enable_overlapping_bulk_in_and_out = true,
    bool has_enable_queued_bulk_in_requests = false,
    bool enable_queued_bulk_in_requests = true,
    bool has_bulk_in_queue_capacity = false,
    int32_t bulk_in_queue_capacity = 32) {
  auto dfu_firmware__ = dfu_firmware ? _fbb.CreateString(dfu_firmware) : 0;
  return platforms::darwinn::api::CreateDriverUsbOptions(
      _fbb,
      dfu_firmware__,
      always_dfu,
      has_fail_if_slower_than_superspeed,
      fail_if_slower_than_superspeed,
      has_force_largest_bulk_in_chunk_size,
      force_largest_bulk_in_chunk_size,
      has_enable_overlapping_bulk_in_and_out,
      enable_overlapping_bulk_in_and_out,
      has_enable_queued_bulk_in_requests,
      enable_queued_bulk_in_requests,
      has_bulk_in_queue_capacity,
      bulk_in_queue_capacity);
}

struct DriverOptions FLATBUFFERS_FINAL_CLASS : private ::flatbuffers::Table {
  typedef DriverOptionsBuilder Builder;
  enum FlatBuffersVTableOffset FLATBUFFERS_VTABLE_UNDERLYING_TYPE {
    VT_VERSION = 4,
    VT_USB = 6,
    VT_VERBOSITY = 8,
    VT_PERFORMANCE_EXPECTATION = 10,
    VT_PUBLIC_KEY = 12,
    VT_WATCHDOG_TIMEOUT_NS = 14,
    VT_TPU_FREQUENCY_HZ = 16,
    VT_MAX_SCHEDULED_WORK_NS = 18,
    VT_HOST_TO_TPU_BPS = 20
  };
  int32_t version() const {
    return GetField<int32_t>(VT_VERSION, 1);
  }
  const platforms::darwinn::api::DriverUsbOptions *usb() const {
    return GetPointer<const platforms::darwinn::api::DriverUsbOptions *>(VT_USB);
  }
  int32_t verbosity() const {
    return GetField<int32_t>(VT_VERBOSITY, 0);
  }
  platforms::darwinn::api::PerformanceExpectation performance_expectation() const {
    return static_cast<platforms::darwinn::api::PerformanceExpectation>(GetField<int32_t>(VT_PERFORMANCE_EXPECTATION, 2));
  }
  const ::flatbuffers::String *public_key() const {
    return GetPointer<const ::flatbuffers::String *>(VT_PUBLIC_KEY);
  }
  int64_t watchdog_timeout_ns() const {
    return GetField<int64_t>(VT_WATCHDOG_TIMEOUT_NS, 0);
  }
  int64_t tpu_frequency_hz() const {
    return GetField<int64_t>(VT_TPU_FREQUENCY_HZ, -1LL);
  }
  int64_t max_scheduled_work_ns() const {
    return GetField<int64_t>(VT_MAX_SCHEDULED_WORK_NS, -1LL);
  }
  int64_t host_to_tpu_bps() const {
    return GetField<int64_t>(VT_HOST_TO_TPU_BPS, -1LL);
  }
  bool Verify(::flatbuffers::Verifier &verifier) const {
    return VerifyTableStart(verifier) &&
           VerifyField<int32_t>(verifier, VT_VERSION, 4) &&
           VerifyOffset(verifier, VT_USB) &&
           verifier.VerifyTable(usb()) &&
           VerifyField<int32_t>(verifier, VT_VERBOSITY, 4) &&
           VerifyField<int32_t>(verifier, VT_PERFORMANCE_EXPECTATION, 4) &&
           VerifyOffset(verifier, VT_PUBLIC_KEY) &&
           verifier.VerifyString(public_key()) &&
           VerifyField<int64_t>(verifier, VT_WATCHDOG_TIMEOUT_NS, 8) &&
           VerifyField<int64_t>(verifier, VT_TPU_FREQUENCY_HZ, 8) &&
           VerifyField<int64_t>(verifier, VT_MAX_SCHEDULED_WORK_NS, 8) &&
           VerifyField<int64_t>(verifier, VT_HOST_TO_TPU_BPS, 8) &&
           verifier.EndTable();
  }
};

struct DriverOptionsBuilder {
  typedef DriverOptions Table;
  ::flatbuffers::FlatBufferBuilder &fbb_;
  ::flatbuffers::uoffset_t start_;
  void add_version(int32_t version) {
    fbb_.AddElement<int32_t>(DriverOptions::VT_VERSION, version, 1);
  }
  void add_usb(::flatbuffers::Offset<platforms::darwinn::api::DriverUsbOptions> usb) {
    fbb_.AddOffset(DriverOptions::VT_USB, usb);
  }
  void add_verbosity(int32_t verbosity) {
    fbb_.AddElement<int32_t>(DriverOptions::VT_VERBOSITY, verbosity, 0);
  }
  void add_performance_expectation(platforms::darwinn::api::PerformanceExpectation performance_expectation) {
    fbb_.AddElement<int32_t>(DriverOptions::VT_PERFORMANCE_EXPECTATION, static_cast<int32_t>(performance_expectation), 2);
  }
  void add_public_key(::flatbuffers::Offset<::flatbuffers::String> public_key) {
    fbb_.AddOffset(DriverOptions::VT_PUBLIC_KEY, public_key);
  }
  void add_watchdog_timeout_ns(int64_t watchdog_timeout_ns) {
    fbb_.AddElement<int64_t>(DriverOptions::VT_WATCHDOG_TIMEOUT_NS, watchdog_timeout_ns, 0);
  }
  void add_tpu_frequency_hz(int64_t tpu_frequency_hz) {
    fbb_.AddElement<int64_t>(DriverOptions::VT_TPU_FREQUENCY_HZ, tpu_frequency_hz, -1LL);
  }
  void add_max_scheduled_work_ns(int64_t max_scheduled_work_ns) {
    fbb_.AddElement<int64_t>(DriverOptions::VT_MAX_SCHEDULED_WORK_NS, max_scheduled_work_ns, -1LL);
  }
  void add_host_to_tpu_bps(int64_t host_to_tpu_bps) {
    fbb_.AddElement<int64_t>(DriverOptions::VT_HOST_TO_TPU_BPS, host_to_tpu_bps, -1LL);
  }
  explicit DriverOptionsBuilder(::flatbuffers::FlatBufferBuilder &_fbb)
        : fbb_(_fbb) {
    start_ = fbb_.StartTable();
  }
  ::flatbuffers::Offset<DriverOptions> Finish() {
    const auto end = fbb_.EndTable(start_);
    auto o = ::flatbuffers::Offset<DriverOptions>(end);
    return o;
  }
};

inline ::flatbuffers::Offset<DriverOptions> CreateDriverOptions(
    ::flatbuffers::FlatBufferBuilder &_fbb,
    int32_t version = 1,
    ::flatbuffers::Offset<platforms::darwinn::api::DriverUsbOptions> usb = 0,
    int32_t verbosity = 0,
    platforms::darwinn::api::PerformanceExpectation performance_expectation = platforms::darwinn::api::PerformanceExpectation_High,
    ::flatbuffers::Offset<::flatbuffers::String> public_key = 0,
    int64_t watchdog_timeout_ns = 0,
    int64_t tpu_frequency_hz = -1LL,
    int64_t max_scheduled_work_ns = -1LL,
    int64_t host_to_tpu_bps = -1LL) {
  DriverOptionsBuilder builder_(_fbb);
  builder_.add_host_to_tpu_bps(host_to_tpu_bps);
  builder_.add_max_scheduled_work_ns(max_scheduled_work_ns);
  builder_.add_tpu_frequency_hz(tpu_frequency_hz);
  builder_.add_watchdog_timeout_ns(watchdog_timeout_ns);
  builder_.add_public_key(public_key);
  builder_.add_performance_expectation(performance_expectation);
  builder_.add_verbosity(verbosity);
  builder_.add_usb(usb);
  builder_.add_version(version);
  return builder_.Finish();
}

inline ::flatbuffers::Offset<DriverOptions> CreateDriverOptionsDirect(
    ::flatbuffers::FlatBufferBuilder &_fbb,
    int32_t version = 1,
    ::flatbuffers::Offset<platforms::darwinn::api::DriverUsbOptions> usb = 0,
    int32_t verbosity = 0,
    platforms::darwinn::api::PerformanceExpectation performance_expectation = platforms::darwinn::api::PerformanceExpectation_High,
    const char *public_key = nullptr,
    int64_t watchdog_timeout_ns = 0,
    int64_t tpu_frequency_hz = -1LL,
    int64_t max_scheduled_work_ns = -1LL,
    int64_t host_to_tpu_bps = -1LL) {
  auto public_key__ = public_key ? _fbb.CreateString(public_key) : 0;
  return platforms::darwinn::api::CreateDriverOptions(
      _fbb,
      version,
      usb,
      verbosity,
      performance_expectation,
      public_key__,
      watchdog_timeout_ns,
      tpu_frequency_hz,
      max_scheduled_work_ns,
      host_to_tpu_bps);
}

inline const platforms::darwinn::api::DriverOptions *GetDriverOptions(const void *buf) {
  return ::flatbuffers::GetRoot<platforms::darwinn::api::DriverOptions>(buf);
}

inline const platforms::darwinn::api::DriverOptions *GetSizePrefixedDriverOptions(const void *buf) {
  return ::flatbuffers::GetSizePrefixedRoot<platforms::darwinn::api::DriverOptions>(buf);
}

inline bool VerifyDriverOptionsBuffer(
    ::flatbuffers::Verifier &verifier) {
  return verifier.VerifyBuffer<platforms::darwinn::api::DriverOptions>(nullptr);
}

inline bool VerifySizePrefixedDriverOptionsBuffer(
    ::flatbuffers::Verifier &verifier) {
  return verifier.VerifySizePrefixedBuffer<platforms::darwinn::api::DriverOptions>(nullptr);
}

inline void FinishDriverOptionsBuffer(
    ::flatbuffers::FlatBufferBuilder &fbb,
    ::flatbuffers::Offset<platforms::darwinn::api::DriverOptions> root) {
  fbb.Finish(root);
}

inline void FinishSizePrefixedDriverOptionsBuffer(
    ::flatbuffers::FlatBufferBuilder &fbb,
    ::flatbuffers::Offset<platforms::darwinn::api::DriverOptions> root) {
  fbb.FinishSizePrefixed(root);
}

}  // namespace api
}  // namespace darwinn
}  // namespace platforms

#endif  // FLATBUFFERS_GENERATED_DRIVEROPTIONS_PLATFORMS_DARWINN_API_H_