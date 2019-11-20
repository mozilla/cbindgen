#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <uchar.h>

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

void root(Foo a);
