from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef int32_t (*DoFn)(int32_t x, int32_t y);

  cdef struct StructWithOptionalFunctionPointer:
    DoFn func;
    DoFn maybe_func;

  ctypedef uint32_t *NonNullAlias_u32;

  cdef struct StructWithOptionalNonNullPointer:
    NonNullAlias_u32 data;
    NonNullAlias_u32 maybe_data;

  void root(StructWithOptionalFunctionPointer swofp, StructWithOptionalNonNullPointer swonnp);
