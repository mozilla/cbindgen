#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct A {
  const int32_t *data;
} A;

typedef enum E_Tag {
  V,
  U,
} E_Tag;

typedef struct U_Body {
  const uint8_t *_0;
} U_Body;

typedef struct E {
  E_Tag tag;
  union {
    U_Body u;
  };
} E;

void root(A _a, E _e);
