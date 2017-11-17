#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct Bar_Bar_f32 Bar_Bar_f32;

typedef struct Bar_Foo_f32 Bar_Foo_f32;

typedef struct Bar_f32 Bar_f32;

typedef struct {
  const int32_t *data;
} Foo_i32;

typedef struct {
  const float *data;
} Foo_f32;

typedef struct {
  const Bar_f32 *data;
} Foo_Bar_f32;

typedef struct {
  const Foo_f32 *a;
  const float *b;
} Tuple_Foo_f32_____f32;

typedef struct {
  const float *a;
  const float *b;
} Indirection_f32;

void root(Foo_i32 a,
          Foo_f32 b,
          Bar_f32 c,
          Foo_Bar_f32 d,
          Bar_Foo_f32 e,
          Bar_Bar_f32 f,
          Tuple_Foo_f32_____f32 g,
          Indirection_f32 h);
