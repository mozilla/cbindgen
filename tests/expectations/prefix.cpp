#include <cstdint>
#include <cstdlib>

static const int32_t PREFIX_LEN = 42;

using PREFIX_NamedLenArray = int32_t[PREFIX_LEN];

using PREFIX_ValuedLenArray = int32_t[42];

extern "C" {

void root(PREFIX_NamedLenArray x, PREFIX_ValuedLenArray y);

} // extern "C"
