from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct Foo_Bar:
    const int32_t *something;

  cdef enum:
    Min,
    Max,
    Other,
  ctypedef uint8_t Bar_Tag;

  cdef struct Min_Body:
    Bar_Tag tag;
    Foo_Bar _0;

  cdef struct Max_Body:
    Bar_Tag tag;
    Foo_Bar _0;

  cdef union Bar:
    Bar_Tag tag;
    Min_Body min;
    Max_Body max;

  void root(Bar b);
