#include <cstdarg>
#include <cstdint>
#include <cstdlib>

/// Constants shared by multiple CSS Box Alignment properties
/// These constants match Gecko's `NS_STYLE_ALIGN_*` constants.
struct StyleAlignFlags {
  uint8_t bits;
  static const StyleAlignFlags AUTO;
  static const StyleAlignFlags NORMAL;
  static const StyleAlignFlags START;
  static const StyleAlignFlags END;
  static const StyleAlignFlags FLEX_START;
};
inline const StyleAlignFlags StyleAlignFlags::AUTO = (StyleAlignFlags){ .bits = 0 };
inline const StyleAlignFlags StyleAlignFlags::NORMAL = (StyleAlignFlags){ .bits = 1 };
inline const StyleAlignFlags StyleAlignFlags::START = (StyleAlignFlags){ .bits = 1 << 1 };
inline const StyleAlignFlags StyleAlignFlags::END = (StyleAlignFlags){ .bits = 1 << 2 };
inline const StyleAlignFlags StyleAlignFlags::FLEX_START = (StyleAlignFlags){ .bits = 1 << 3 };

extern "C" {

void root(StyleAlignFlags flags);

} // extern "C"
