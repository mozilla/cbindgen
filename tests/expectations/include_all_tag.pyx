from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef enum:
    A,
    B,
  ctypedef uint8_t UnusedEnum;

  cdef struct ExplicitOpaque:
    pass

  cdef struct OpaqueDependency:
    pass

  cdef struct UnusedStruct:
    int32_t x;
    float y;

  ctypedef uint32_t UnusedTransparent;

  cdef union UnusedUnion:
    int32_t x;
    float y;

  ctypedef int32_t UnusedAlias;

  cdef struct UsesOpaqueDependency:
    OpaqueDependency *opaque;

  cdef struct PublicInPrivateModule:
    int32_t x;
