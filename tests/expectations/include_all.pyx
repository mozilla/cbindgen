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

  ctypedef struct ExplicitOpaque:
    pass

  ctypedef struct OpaqueDependency:
    pass

  ctypedef struct UnusedStruct:
    int32_t x;
    float y;

  ctypedef uint32_t UnusedTransparent;

  ctypedef union UnusedUnion:
    int32_t x;
    float y;

  ctypedef int32_t UnusedAlias;

  ctypedef struct UsesOpaqueDependency:
    OpaqueDependency *opaque;

  ctypedef struct PublicInPrivateModule:
    int32_t x;
