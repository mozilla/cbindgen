#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

static const int32_t PREFIX_LEN = 42;

using PREFIX_NamedLenArray = int32_t[PREFIX_LEN];

using PREFIX_ValuedLenArray = int32_t[42];

union PREFIX_AbsoluteFontWeight {
  enum class Tag : uint8_t {
    Weight,
    Normal,
    Bold,
  };

  struct Weight_Body {
    Tag tag;
    float _0;
  };

  struct {
    Tag tag;
  };
  Weight_Body weight;
};

extern "C" {

void root(PREFIX_NamedLenArray x, PREFIX_ValuedLenArray y, PREFIX_AbsoluteFontWeight z);

} // extern "C"
