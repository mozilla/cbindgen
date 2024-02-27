''' Package version: 0.1.0 '''

''' Text to put at the beginning of the file. Probably a license. '''

''' Test autogen warning '''

from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list
''' Text after includes '''

cdef extern from *:

  cdef struct Foo:
    pass

  void doit(const Foo*);

''' Text to put at the end of the file. '''
