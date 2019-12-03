#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define EXPORT_ME_TOO 42

typedef struct {
  uint64_t val;
} ExportMe;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void export_me(ExportMe *val);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
