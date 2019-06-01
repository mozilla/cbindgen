#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum {
  A,
} Foo_Tag;

typedef struct {
  float _0[20];
} A_Body;

typedef struct {
  Foo_Tag tag;
  union {
    A_Body a;
  };
} Foo;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(Foo a);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
