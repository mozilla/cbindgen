from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef enum COption_Tag:
    Some,
    None,

  ctypedef struct COption_u8:
    COption_Tag tag;
    uint8_t some;

  ctypedef struct COption_u32:
    COption_Tag tag;
    uint32_t some;

  void root(COption_u8 a, COption_u32 b);
