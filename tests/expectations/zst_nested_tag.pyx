from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct Opaque_1Z:
    pass

  cdef struct Opaque_______c_void:
    pass

  cdef enum MyOption_______c_void_Tag:
    Some_______c_void,
    None_______c_void,

  cdef struct MyOption_______c_void:
    MyOption_______c_void_Tag tag;
    void (*some)();

  cdef struct MyWrapper_______c_void:
    void (*inner)();

  cdef struct S_1Z:
    void (*args_erased)();
    void (*args_erased_with_npo)();
    MyOption_______c_void args_erased_with_custom_enum;
    MyWrapper_______c_void args_erased_with_custom_struct;
    const Opaque_______c_void *args_erased_with_opaque;
    void (**args_erased_behind_ptr)();
    void (*args_erased_in_array[2])();
    void (*(*args_erased_in_ret)())();
    const void *arr_erased_behind_ptr;
    const Opaque_1Z *arr_erased_in_opaque;

  cdef struct S2_1Z___________u8__________4:
    void (*f)(uint8_t[4]);
    uint8_t x[4];

  cdef struct Inner_1Z:
    const void *p;

  cdef struct Outer_1Z:
    Inner_1Z plain;
    Inner_1Z arr;

  void use_S(S_1Z arg1, S2_1Z___________u8__________4 arg2, Outer_1Z arg3);
