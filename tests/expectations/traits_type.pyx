from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct Dummy0:
    uintptr_t dummy;

  ctypedef struct Dummy1:
    uintptr_t dummy;

  Dummy0 dummy_Dummy0(Dummy0 self, uintptr_t in_);

  int32_t dummy_Dummy1(Dummy1 self);
