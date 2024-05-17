from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct Option_Option_i32:
    pass

  cdef struct Option_i64:
    pass

  cdef struct AlwaysErased1_i32:
    int32_t a;
    int32_t *n;
    int32_t t;

  cdef struct SometimesErased1_____i16:
    int16_t *const *o;

  cdef struct SometimesErased1_i32:
    const int32_t *o;

  cdef struct SometimesErased1_i64:
    const Option_i64 *o;

  cdef struct AlwaysErased2_i32:
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

  cdef struct SometimesErased2_____i16:
    int16_t *const *ao;
    int16_t **const *no;
    int16_t *const *oa;
    int16_t *const *ot;
    int16_t *const *to;

  cdef struct SometimesErased2_i32:
    const int32_t *ao;
    int32_t *const *no;
    const int32_t *oa;
    const int32_t *ot;
    const int32_t *to;

  cdef struct SometimesErased2_i64:
    const Option_i64 *ao;
    Option_i64 *const *no;
    const Option_i64 *oa;
    const Option_i64 *ot;
    const Option_i64 *to;

  cdef struct NeverErased2_i32:
    const Option_Option_i32 *oo;

  cdef struct AlwaysErasedMany_i32:
    int32_t *tont;
    int32_t *otnt;
    int32_t *totn;
    int32_t *totnt;

  void root1(AlwaysErased1_i32 a,
             SometimesErased1_____i16 sn,
             SometimesErased1_i32 sz,
             SometimesErased1_i64 si);

  void root2(AlwaysErased2_i32 a,
             SometimesErased2_____i16 sn,
             SometimesErased2_i32 sz,
             SometimesErased2_i64 si,
             NeverErased2_i32 n);

  void root_many(AlwaysErasedMany_i32 a);
