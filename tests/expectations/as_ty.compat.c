#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define SIZE 4

typedef struct {
  uint32_t items[SIZE];
} WithoutAs;

typedef struct {
  uint32_t items[SIZE];
} WithAs;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void some_fn(WithoutAs a, WithAs b);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
