#if 0
DEF PLATFORM_UNIX = 0
DEF PLATFORM_WIN = 0
DEF X11 = 0
DEF M_32 = 0
#endif


from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  IF PLATFORM_UNIX:
    const uint32_t FOO_CONST # = 0

  IF PLATFORM_WIN:
    const uint32_t FOO_CONST # = 1

  IF not (PLATFORM_UNIX or PLATFORM_WIN):
    const uint32_t FOO_CONST # = 61453

  IF PLATFORM_UNIX:
    const uint8_t BAR_CONST # = 0

  IF PLATFORM_WIN:
    const uint8_t BAR_CONST # = raise TypeError("reached unreachable code")

  IF not (PLATFORM_UNIX or PLATFORM_WIN):
    const uint8_t BAR_CONST # = 1

  IF (X11 and PLATFORM_UNIX):
    const uint8_t BAZ_CONST # = 0

  IF (X11 and PLATFORM_WIN):
    const uint8_t BAZ_CONST # = 1

  IF (X11 and not (PLATFORM_UNIX or PLATFORM_WIN)):
    const uint8_t BAZ_CONST # = raise TypeError("Baz error")

  IF (PLATFORM_WIN or M_32):
    cdef enum:
      A,
      B,
      C,
    ctypedef uint32_t BarType;

  IF (PLATFORM_UNIX and X11):
    cdef enum:
      A,
      B,
      C,
    ctypedef uint32_t FooType;

  IF (PLATFORM_UNIX and X11):
    cdef struct FooHandle:
      FooType ty;
      int32_t x;
      float y;

  cdef enum:
    C1,
    C2,
    C3,
    C5,
  ctypedef uint8_t C_Tag;

  cdef struct C5_Body:
    C_Tag tag;
    int32_t int_;

  cdef union C:
    C_Tag tag;
    C5_Body c5;

  IF (PLATFORM_WIN or M_32):
    cdef struct BarHandle:
      BarType ty;
      int32_t x;
      float y;

  cdef struct ConditionalField:
    int32_t field;

  cdef struct Normal:
    int32_t x;
    float y;

  IF PLATFORM_WIN:
    extern int32_t global_array_with_different_sizes[2];

  IF PLATFORM_UNIX:
    extern int32_t global_array_with_different_sizes[1];

  IF (PLATFORM_UNIX and X11):
    void root(FooHandle a, C c);

  IF (PLATFORM_WIN or M_32):
    void root(BarHandle a, C c);

  void cond(ConditionalField a);

  IF PLATFORM_WIN:
    extern int32_t foo();

  IF PLATFORM_WIN:
    extern void bar(Normal a);
