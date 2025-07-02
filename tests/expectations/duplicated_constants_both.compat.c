#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Foo {
  uint32_t field;
} Foo;
#define Foo_FIELD_RELATED_CONSTANT 0

typedef struct Bar {
  uint32_t field;
} Bar;
#define Bar_FIELD_RELATED_CONSTANT 0

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct Foo a, struct Bar b);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
