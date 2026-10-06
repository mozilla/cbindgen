from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct Opaque_1Z:
    pass

  ctypedef struct Option_1Z:
    pass

  ctypedef struct Option_Option_1Z:
    pass

  ctypedef struct Option_u32:
    pass

  ctypedef struct Result_1Z__1Z:
    pass

  ctypedef struct Result_1Z__u32:
    pass

  ctypedef struct Result_u32__1Z:
    pass

  ctypedef struct CG_u8__3:
    uint8_t a[3];
    uint32_t b;

  ctypedef struct ValidPositions_1Z:
    void (*args_always_erased)();
    const void *zst_behind_ptr;
    const Opaque_1Z *zst_in_opaque;
    void (*f_returns_zst)(uint32_t arg);

  ctypedef struct APIError:
    uint32_t err;

  ctypedef struct CResultTempl_1Z__APIError:
    bool result_good;
    const void *result;
    const APIError *err;

  ctypedef CResultTempl_1Z__APIError CResultNoneAPIError;

  ctypedef struct Foo_1Z:
    uint32_t x;

  void option_u32(Option_u32 arg);

  void option_unit(Option_1Z arg, Option_Option_1Z nested);

  void result_zsts(Result_1Z__u32 arg1, Result_u32__1Z arg2, Result_1Z__1Z arg3);

  void use_cg(CG_u8__3 x);

  void use_valid_positions(ValidPositions_1Z arg);

  void CResultNoneAPIError_free(CResultNoneAPIError _res);

  Foo_1Z bar();
