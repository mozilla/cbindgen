#include <cstdarg>
#include <cstdint>
#include <cstdlib>

struct ExtType {
  uint32_t data;
};

extern "C" {

void consume_ext(ExtType _ext);

} // extern "C"
