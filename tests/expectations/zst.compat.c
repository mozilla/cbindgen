#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  void *data;
  void *vtable;
} TraitObject;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void *root(const void *ptr, TraitObject t);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
