#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  void (*noArgs)(void);
  void (*anonymousArg)(int32_t);
  int32_t (*returnsNumber)(void);
  int8_t (*namedArgs)(int32_t first, int16_t snd);
  int8_t (*namedArgsWildcards)(int32_t _, int16_t named, int64_t _1);
} Fns;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(Fns _fns);

void no_return(void);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
