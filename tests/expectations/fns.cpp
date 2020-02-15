#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

struct Fns {
  void (*noArgs)(void);
  void (*anonymousArg)(int32_t);
  int32_t (*returnsNumber)(void);
  int8_t (*namedArgs)(int32_t first, int16_t snd);
  int8_t (*namedArgsWildcards)(int32_t _, int16_t named, int64_t _1);
};

extern "C" {

void root(Fns _fns);

} // extern "C"
