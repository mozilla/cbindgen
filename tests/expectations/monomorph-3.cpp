#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

template<typename T = void>
struct Bar;

template<typename T>
union Foo {
  const T *data;
};

template<typename T, typename E>
union Tuple {
  const T *a;
  const E *b;
};

template<typename T>
using Indirection = Tuple<T, float>;

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
