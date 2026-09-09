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

  ctypedef struct P:
    pass

  ctypedef struct T:
    pass

  ctypedef uint8_t *BytePtr;

  ctypedef uint16_t *NumberPtr;

  void root(A x, B y, C z, F f, H h, P p, T t);

  void bytes_and_nums(BytePtr b, NumberPtr n, BytePtr *bp, NumberPtr *np);
