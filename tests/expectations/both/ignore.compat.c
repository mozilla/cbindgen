#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct OneFieldIgnored {
  int32_t x;
  int32_t z;
} OneFieldIgnored;

typedef struct AllFieldsIgnored {

} AllFieldsIgnored;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void no_ignore_root(OneFieldIgnored one, AllFieldsIgnored all);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
