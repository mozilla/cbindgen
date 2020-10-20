#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  const int32_t *data;
} A;

typedef enum {
  V,
  U,
} E_Tag;

typedef struct {
  const uint8_t *_0;
} U_Body;

typedef struct {
  E_Tag tag;
  union {
    U_Body u;
  };
} E;

void root(A _a, E _e);
