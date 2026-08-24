from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct Payload:
    int32_t x;

  ctypedef enum ResultA_Payload_Tag:
    ResultA_Payload_Ok_Payload,
    ResultA_Payload_Err_Payload,

  ctypedef struct ResultA_Payload:
    ResultA_Payload_Tag tag;
    Payload ok;
    void *err;

  ctypedef enum ResultB_Payload_Tag:
    ResultB_Payload_Ok_Payload,
    ResultB_Payload_Err_Payload,

  ctypedef struct ResultB_Payload:
    ResultB_Payload_Tag tag;
    Payload ok;
    void *err;

  void use_a(ResultA_Payload _a);

  void use_b(ResultB_Payload _b);
