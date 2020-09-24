#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  int32_t x;
  int32_t z;
} OneFieldIgnored;

typedef struct {

} AllFieldsIgnored;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void no_ignore_root(OneFieldIgnored one, AllFieldsIgnored all);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
