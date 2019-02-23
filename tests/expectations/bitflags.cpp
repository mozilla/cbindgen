#include <cstdarg>
#include <cstdint>
#include <cstdlib>

/// Constants shared by multiple CSS Box Alignment properties
/// These constants match Gecko's `NS_STYLE_ALIGN_*` constants.
struct AlignFlags {
  uint8_t bits;
};

static const AlignFlags AlignFlags_AUTO = (AlignFlags){ .bits = 0 };

static const AlignFlags AlignFlags_END = (AlignFlags){ .bits = 1 << 2 };

static const AlignFlags AlignFlags_FLEX_START = (AlignFlags){ .bits = 1 << 3 };

static const AlignFlags AlignFlags_NORMAL = (AlignFlags){ .bits = 1 };

static const AlignFlags AlignFlags_START = (AlignFlags){ .bits = 1 << 1 };
