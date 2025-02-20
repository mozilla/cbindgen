from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  const intptr_t SIZE # = 4

  cdef struct WithoutAs:
    uint32_t items[SIZE];

  cdef struct WithAs:
    uint32_t items[SIZE];

  void some_fn(WithoutAs a, WithAs b);
