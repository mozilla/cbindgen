#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define EXPORT_ME_TOO 42

typedef struct ExportMe {
  uint64_t val;
} ExportMe;

typedef struct ExportMe2 {
  uint64_t val;
} ExportMe2;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void export_me(struct ExportMe *val);

void export_me_2(struct ExportMe2*);

void from_really_nested_mod(void);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
