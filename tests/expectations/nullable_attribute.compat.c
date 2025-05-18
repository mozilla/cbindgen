#ifdef __clang__
#define CBINDGEN_NULLABLE _Nullable
#else
#define CBINDGEN_NULLABLE
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Opaque Opaque;

typedef struct {
  const Opaque *a;
  Opaque *b;
  const Opaque *CBINDGEN_NULLABLE c;
  Opaque *CBINDGEN_NULLABLE d;
} References;

typedef struct {
  float *a;
  uint64_t *b;
  Opaque *c;
  uint64_t **d;
  float **e;
  Opaque **f;
  uint64_t *CBINDGEN_NULLABLE g;
  int32_t *CBINDGEN_NULLABLE h;
  int32_t **CBINDGEN_NULLABLE i;
  const uint64_t *CBINDGEN_NULLABLE j;
  uint64_t *CBINDGEN_NULLABLE k;
} Pointers_u64;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void value_arg(References arg);

void mutltiple_args(int32_t *arg, Pointers_u64 *CBINDGEN_NULLABLE foo, Opaque **d);

void ref_arg(const Pointers_u64 *arg);

void mut_ref_arg(Pointers_u64 *arg);

void optional_ref_arg(const Pointers_u64 *CBINDGEN_NULLABLE arg);

void optional_mut_ref_arg(Pointers_u64 *CBINDGEN_NULLABLE arg);

void nullable_const_ptr(const Pointers_u64 *CBINDGEN_NULLABLE arg);

void nullable_mut_ptr(Pointers_u64 *CBINDGEN_NULLABLE arg);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
