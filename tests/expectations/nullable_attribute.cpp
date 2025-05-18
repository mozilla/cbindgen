#ifdef __clang__
#define CBINDGEN_NULLABLE _Nullable
#else
#define CBINDGEN_NULLABLE
#endif


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Opaque;

struct References {
  const Opaque *a;
  Opaque *b;
  const Opaque *CBINDGEN_NULLABLE c;
  Opaque *CBINDGEN_NULLABLE d;
};

template<typename T>
struct Pointers {
  float *a;
  T *b;
  Opaque *c;
  T **d;
  float **e;
  Opaque **f;
  T *CBINDGEN_NULLABLE g;
  int32_t *CBINDGEN_NULLABLE h;
  int32_t **CBINDGEN_NULLABLE i;
  const T *CBINDGEN_NULLABLE j;
  T *CBINDGEN_NULLABLE k;
};

extern "C" {

void value_arg(References arg);

void mutltiple_args(int32_t *arg, Pointers<uint64_t> *CBINDGEN_NULLABLE foo, Opaque **d);

void ref_arg(const Pointers<uint64_t> *arg);

void mut_ref_arg(Pointers<uint64_t> *arg);

void optional_ref_arg(const Pointers<uint64_t> *CBINDGEN_NULLABLE arg);

void optional_mut_ref_arg(Pointers<uint64_t> *CBINDGEN_NULLABLE arg);

void nullable_const_ptr(const Pointers<uint64_t> *CBINDGEN_NULLABLE arg);

void nullable_mut_ptr(Pointers<uint64_t> *CBINDGEN_NULLABLE arg);

}  // extern "C"
