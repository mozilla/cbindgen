from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef void (*C_1Z)(uint8_t);

  ctypedef const void *D_1Z;

  ctypedef struct Wrapper_1Z:
    C_1Z c;
    D_1Z d;

  void use_typedefs_ptr(const Wrapper_1Z *w);
