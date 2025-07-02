from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct Foo:
    uint32_t field;
  const uint32_t Foo_FIELD_RELATED_CONSTANT # = 0

  ctypedef struct Bar:
    uint32_t field;
  const uint32_t Bar_FIELD_RELATED_CONSTANT # = 0

  void root(Foo a, Bar b);
