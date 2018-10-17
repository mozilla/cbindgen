#include <cstdint>
#include <cstdlib>

struct Opaque;

template<typename T>
struct Foo {
  float *a;
  T *b;
  Opaque *c;
  T **d;
  float **e;
  Opaque **f;
  T *g;
  int32_t *h;
  int32_t **i;
};

extern "C" {

void root(int32_t *arg, Foo<uint64_t> *foo, Opaque **d);

} // extern "C"
