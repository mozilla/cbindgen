from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct Foo_i16:
    int16_t x;

  cdef struct Foo_i8:
    int8_t x;

  cdef struct NotReturnValue_i32:
    int32_t x;

  cdef struct FooField:
    Foo_i8 (*f)();
    void (*g)(NotReturnValue_i32);

  cdef struct Bar_i16__i16:
    int16_t p;
    int16_t q;

  cdef struct Bar_i8__i32:
    int8_t p;
    int32_t q;

  ctypedef Bar_i8__i32 IntBar_i32;

  cdef struct Bar_i8__bool:
    int8_t p;
    bool q;

  ctypedef Bar_i8__bool IntBar_bool;

  ctypedef IntBar_bool IntBoolBar;

  cdef struct Foo_i32:
    int32_t x;

  ctypedef Foo_i32 WrapFoo_i32;

  cdef struct Bar_bool__bool:
    bool p;
    bool q;

  ctypedef Bar_bool__bool BoolBoolBar;

  ctypedef BoolBoolBar WrapBoolBoolBar;

  cdef struct Foo_bool:
    bool x;

  ctypedef int8_t WrapNonZeroInt;

  cdef struct Foo_i64:
    int64_t x;

  ctypedef Foo_i64 Transparent;

  int32_t fnA();

  int16_t fnB();

  Foo_i16 fnE();

  void fnF(FooField f);

  Bar_i16__i16 fnG();

  IntBar_i32 fnH();

  IntBoolBar fnI();

  WrapFoo_i32 fnJ();

  WrapBoolBoolBar fnK();

  Foo_bool fnL();

  WrapNonZeroInt fnM();

  Transparent fnN();
