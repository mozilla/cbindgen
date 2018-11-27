#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  int32_t namespace_;
  float float_;
} A;

typedef struct {
  int32_t namespace_;
  float float_;
} B;

enum C_Tag {
  D,
};
typedef uint8_t C_Tag;

typedef struct {
  int32_t namespace_;
  float float_;
} D_Body;

typedef struct {
  C_Tag tag;
  union {
    D_Body d;
  };
} C;

void root(A a, B b, C c, int32_t namespace_, float float_);
