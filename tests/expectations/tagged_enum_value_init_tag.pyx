from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  # Regression test: in C++ output, the generated static variant constructors of a
  # tagged enum must value-initialize their `result` local (`Foo result{};`) so that
  # the payload union is zero-initialized rather than left indeterminate. This avoids
  # false "uninitialized scalar" reports from static analysis for no-payload variants.
  #
  cdef enum:
    # No payload: `result` is returned with only the tag set, so the union must be
    # value-initialized.
    Empty,
    # Payload variant: constructed via placement new over the value-initialized union.
    Number,
    Pair,
  ctypedef uint8_t ValueInitEnum_Tag;

  cdef struct Pair_Body:
    int32_t _0;
    uint64_t _1;

  cdef struct ValueInitEnum:
    ValueInitEnum_Tag tag;
    int32_t number;
    Pair_Body pair;

  void root(ValueInitEnum e);
