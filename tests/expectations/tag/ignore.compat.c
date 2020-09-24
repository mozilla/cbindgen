#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct OneFieldIgnored {
  int32_t x;
  int32_t z;
};

struct AllFieldsIgnored {

};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void no_ignore_root(struct OneFieldIgnored one, struct AllFieldsIgnored all);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
