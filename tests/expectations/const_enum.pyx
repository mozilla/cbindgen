from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef enum:
    Nonzero,
    Evenodd,
  ctypedef uint8_t FillRule;

  ctypedef struct Style:
    FillRule rule;
  const FillRule Style_DEFAULT_RULE # = Nonzero
  const FillRule Style_ALL_RULES[2] # = [ Nonzero, Evenodd, ]

  const FillRule DEFAULT_FILL_RULE # = Nonzero

  const FillRule ALL_FILL_RULES[2] # = [ Nonzero, Evenodd, ]

  void root(FillRule rule, Style style);
