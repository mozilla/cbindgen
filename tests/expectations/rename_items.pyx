from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  const int32_t capi_constant_abc # = 10

  cdef enum:
    capi_enumeration_x # = 0,
    capi_enumeration_y # = 1,
  ctypedef uint8_t capi_enumeration;

  ctypedef struct capi_struct_abc:
    pass

  ctypedef struct capi_union_ghi:
    pass

  ctypedef struct capi_struct_def:
    int32_t x;
    float y;

  ctypedef union capi_union_jkl:
    int32_t x;
    float y;

  ctypedef capi_struct_abc capi_type_alias;

  const intptr_t capi_constant_expression # = <intptr_t><capi_type_alias*>10

  extern const int32_t StaticAbc;

  void root(const capi_struct_abc *a,
            capi_struct_def b,
            capi_union_ghi c,
            capi_union_jkl d,
            capi_enumeration e,
            capi_type_alias f);
