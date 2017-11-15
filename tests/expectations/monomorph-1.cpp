#include <cstdint>
#include <cstdlib>

template<typename T>
struct Bar;

template<typename T>
struct Foo {
  const T *data;
};

template<typename T, typename E>
struct Tuple {
  const T *a;
  const E *b;
};

template<typename T>
struct Indirection {
  const T *a;
  const float *b;
};

extern "C" {

void root(Foo<int32_t> a,
          Foo<float> b,
          Bar<float> c,
          Foo<Bar<float>> d,
          Bar<Foo<float>> e,
          Bar<Bar<float>> f,
          Tuple<Foo<float>, float> g,
          Indirection<float> h);

} // extern "C"
