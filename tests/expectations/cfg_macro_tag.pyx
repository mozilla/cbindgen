from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct Collision:
    int32_t x;
    int32_t y;

  cdef struct Nested:
    Collision collision;
    const uint8_t *pointer;

  cdef struct Foo:
    int32_t x;

  const Nested NESTED # = <Nested>{ <Collision>{ 1, 2 }, <const uint8_t*>0 }

  const Foo FOO # = <Foo>{ 0 }
