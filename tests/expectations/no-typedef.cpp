#include <cstdint>
#include <cstdlib>

struct simple {
  uint64_t len;
};

extern "C" {

const struct simple *simple(const struct simple *simple);

} // extern "C"
