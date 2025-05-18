#ifdef __clang__
#define CBINDGEN_NULLABLE _Nullable
#else
#define CBINDGEN_NULLABLE
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Opaque;

struct References {
  const struct Opaque *a;
  struct Opaque *b;
  const struct Opaque *CBINDGEN_NULLABLE c;
  struct Opaque *CBINDGEN_NULLABLE d;
};

struct Pointers_u64 {
  float *a;
  uint64_t *b;
  struct Opaque *c;
  uint64_t **d;
  float **e;
  struct Opaque **f;
  uint64_t *CBINDGEN_NULLABLE g;
  int32_t *CBINDGEN_NULLABLE h;
  int32_t **CBINDGEN_NULLABLE i;
  const uint64_t *CBINDGEN_NULLABLE j;
  uint64_t *CBINDGEN_NULLABLE k;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void value_arg(struct References arg);

void mutltiple_args(int32_t *arg, struct Pointers_u64 *CBINDGEN_NULLABLE foo, struct Opaque **d);

void ref_arg(const struct Pointers_u64 *arg);

void mut_ref_arg(struct Pointers_u64 *arg);

void optional_ref_arg(const struct Pointers_u64 *CBINDGEN_NULLABLE arg);

void optional_mut_ref_arg(struct Pointers_u64 *CBINDGEN_NULLABLE arg);

void nullable_const_ptr(const struct Pointers_u64 *CBINDGEN_NULLABLE arg);

void nullable_mut_ptr(struct Pointers_u64 *CBINDGEN_NULLABLE arg);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
