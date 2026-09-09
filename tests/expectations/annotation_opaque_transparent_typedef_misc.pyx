from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct A:
    pass

  ctypedef struct B:
    pass

  ctypedef struct C:
    pass

  ctypedef struct F:
    pass

  ctypedef struct H:
    pass

  ctypedef struct O:
    pass

  ctypedef struct P:
    pass

  void root(P p, P *pp, uint16_t t, uint16_t *tp, O o, O *op);

  void bytes_and_nums(uint8_t *b, uint16_t *n, uint8_t **bp, uint16_t **np);

  void other(A x, B y, C z, F f, H h);
