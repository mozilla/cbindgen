#include <cstdint>
#include <cstdlib>

extern "C" {

struct Bar_Bar_f32;

struct Bar_Foo_f32;

struct Bar_f32;

struct Foo_i32 {
  const int32_t *data;
};

struct Foo_f32 {
  const float *data;
};

struct Foo_Bar_f32 {
  const Bar_f32 *data;
};

struct Tuple_Foo_f32_____f32 {
  const Foo_f32 *a;
  const float *b;
};

struct Indirection_f32 {
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

template<typename T>
struct Foo;

template<>
struct Foo<int32_t> : public Foo_i32 {

};

template<>
struct Foo<float> : public Foo_f32 {

};

template<>
struct Foo<Bar_f32> : public Foo_Bar_f32 {

};

template<typename T>
struct Indirection;

template<>
struct Indirection<float> : public Indirection_f32 {

};

template<typename T, typename E>
struct Tuple;

template<>
struct Tuple<Foo_f32, float> : public Tuple_Foo_f32_____f32 {

};
