#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

/// Constants shared by multiple CSS Box Alignment properties
///
/// These constants match Gecko's `NS_STYLE_ALIGN_*` constants.
struct AlignFlags {
  uint8_t bits;

  explicit operator bool() const {
    return !!bits;
  }
  AlignFlags operator|(const AlignFlags& other) const {
    return {static_cast<decltype(bits)>(this->bits | other.bits)};
  }
  AlignFlags& operator|=(const AlignFlags& other) {
    *this = (*this | other);
    return *this;
  }
  AlignFlags operator&(const AlignFlags& other) const {
    return {static_cast<decltype(bits)>(this->bits & other.bits)};
  }
  AlignFlags& operator&=(const AlignFlags& other) {
    *this = (*this & other);
    return *this;
  }
};
static const AlignFlags AlignFlags_AUTO = { /* .bits = */ 0 };
static const AlignFlags AlignFlags_NORMAL = { /* .bits = */ 1 };
static const AlignFlags AlignFlags_START = { /* .bits = */ 1 << 1 };
static const AlignFlags AlignFlags_END = { /* .bits = */ 1 << 2 };
static const AlignFlags AlignFlags_FLEX_START = { /* .bits = */ 1 << 3 };

extern "C" {

void root(AlignFlags flags);

} // extern "C"
