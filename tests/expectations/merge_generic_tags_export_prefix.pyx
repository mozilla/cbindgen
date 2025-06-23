from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef enum Prefix_COption_Tag:
    PREFIX_C_OPTION_SOME,
    PREFIX_C_OPTION_NONE,


  ctypedef enum Prefix_CResult_Tag:
    PREFIX_C_RESULT_OK,
    PREFIX_C_RESULT_ERR,




  ctypedef struct Prefix_COption_u32:
    Prefix_COption_Tag tag;
    uint32_t some;

  ctypedef struct Prefix_ErrorInfo:
    int32_t code;
    const uint8_t *message;



  ctypedef struct Prefix_CResult_u32__ErrorInfo:
    Prefix_CResult_Tag tag;
    uint32_t ok;
    Prefix_ErrorInfo err;



  ctypedef struct Prefix_COption______u8:
    Prefix_COption_Tag tag;
    const uint8_t *some;



  ctypedef struct Prefix_CResult______u8__i32:
    Prefix_CResult_Tag tag;
    const uint8_t *ok;
    int32_t err;



  ctypedef struct Prefix_COption_i32:
    Prefix_COption_Tag tag;
    int32_t some;

  Prefix_COption_u32 process_result(Prefix_CResult_u32__ErrorInfo r);

  Prefix_COption______u8 process_str_result(Prefix_CResult______u8__i32 r);

  Prefix_COption_i32 get_option_int();

  Prefix_COption______u8 get_option_str();
