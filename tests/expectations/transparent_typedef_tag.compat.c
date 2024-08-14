#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Option_Option_i32;

struct Option_i64;

struct AlwaysErased1_i32 {
  int32_t a;
  int32_t *n;
  int32_t t;
};

struct SometimesErased1_____i16 {
  int16_t *const *o;
};

struct SometimesErased1_i32 {
  const int32_t *o;
};

struct SometimesErased1_i64 {
  const struct Option_i64 *o;
};

struct AlwaysErased2_i32 {
  int32_t aa;
  int32_t *an;
  int32_t at;
  int32_t *na;
  int32_t **nn;
  int32_t *nt;
  int32_t *on;
  int32_t ta;
  int32_t *tn;
  int32_t tt;
};

struct SometimesErased2_____i16 {
  int16_t *const *ao;
  int16_t **const *no;
  int16_t *const *oa;
  int16_t *const *ot;
  int16_t *const *to;
};

struct SometimesErased2_i32 {
  const int32_t *ao;
  int32_t *const *no;
  const int32_t *oa;
  const int32_t *ot;
  const int32_t *to;
};

struct SometimesErased2_i64 {
  const struct Option_i64 *ao;
  struct Option_i64 *const *no;
  const struct Option_i64 *oa;
  const struct Option_i64 *ot;
  const struct Option_i64 *to;
};

struct NeverErased2_i32 {
  const struct Option_Option_i32 *oo;
};

struct AlwaysErasedMany_i32 {
  int32_t *tont;
  int32_t *otnt;
  int32_t *totn;
  int32_t *totnt;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root1(struct AlwaysErased1_i32 a,
           struct SometimesErased1_____i16 sn,
           struct SometimesErased1_i32 sz,
           struct SometimesErased1_i64 si);

void root2(struct AlwaysErased2_i32 a,
           struct SometimesErased2_____i16 sn,
           struct SometimesErased2_i32 sz,
           struct SometimesErased2_i64 si,
           struct NeverErased2_i32 n);

void root_many(struct AlwaysErasedMany_i32 a);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
