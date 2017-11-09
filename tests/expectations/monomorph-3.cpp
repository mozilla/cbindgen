#include <cstdint>
#include <cstdlib>

extern "C" {

struct Bar_Bar_f32;

struct Bar_Foo_f32;

struct Bar_f32;

union Foo_i32 {
  const int32_t *data;
};

union Foo_f32 {
  const float *data;
};

union Foo_Bar_f32 {
  const Bar_f32 *data;
};

union Tuple_Foo_f32_____f32 {
  const Foo_f32 *a;
  const float *b;
};

union Indirection_f32 {
  const float *a;
  const float *b;
};

void root(Foo_i32 a,
          Foo_f32 b,
          Bar_f32 c,
          Foo_Bar_f32 d,
          Bar_Foo_f32 e,
          Bar_Bar_f32 f,
          Tuple_Foo_f32_____f32 g,
          Indirection_f32 h);

} // extern "C"
