from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef enum MyCLikeEnum:
    Foo1,
    Bar1,
    Baz1,

  ctypedef enum MyCLikeEnum_Prepended:
    Foo1_Prepended,
    Bar1_Prepended,
    Baz1_Prepended,

  ctypedef struct MyFancyStruct:
    int32_t i;
#ifdef __cplusplus
    inline void foo();
#endif

  ctypedef enum MyFancyEnum_Tag:
    Foo,
    Bar,
    Baz,

  ctypedef struct Bar_Body:
    int32_t _0;

  ctypedef struct Baz_Body:
    int32_t _0;

  ctypedef struct MyFancyEnum:
    MyFancyEnum_Tag tag;
    Bar_Body bar;
    Baz_Body baz;
#ifdef __cplusplus
    inline void wohoo();
#endif

  ctypedef union MyUnion:
    float f;
    uint32_t u;
    int32_t extra_member;

  ctypedef struct MyFancyStruct_Prepended:
#ifdef __cplusplus
  inline void prepended_wohoo();
#endif
    int32_t i;

  ctypedef enum MyFancyEnum_Prepended_Tag:
    Foo_Prepended,
    Bar_Prepended,
    Baz_Prepended,

  ctypedef struct Bar_Prepended_Body:
    int32_t _0;

  ctypedef struct Baz_Prepended_Body:
    int32_t _0;

  ctypedef struct MyFancyEnum_Prepended:
  #ifdef __cplusplus
    inline void wohoo();
  #endif
    MyFancyEnum_Prepended_Tag tag;
    Bar_Prepended_Body bar_prepended;
    Baz_Prepended_Body baz_prepended;

  ctypedef union MyUnion_Prepended:
    int32_t extra_member;
    float f;
    uint32_t u;

  void root(MyFancyStruct s,
            MyFancyEnum e,
            MyCLikeEnum c,
            MyUnion u,
            MyFancyStruct_Prepended sp,
            MyFancyEnum_Prepended ep,
            MyCLikeEnum_Prepended cp,
            MyUnion_Prepended up);
