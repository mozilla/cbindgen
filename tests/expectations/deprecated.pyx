#define DEPRECATED_FUNC __attribute__((deprecated))
#define DEPRECATED_STRUCT __attribute__((deprecated))
#define DEPRECATED_ENUM __attribute__((deprecated))
#define DEPRECATED_FUNC_WITH_NOTE(...) __attribute__((deprecated(__VA_ARGS__)))
#define DEPRECATED_STRUCT_WITH_NOTE(...) __attribute__((deprecated(__VA_ARGS__)))
#define DEPRECATED_ENUM_WITH_NOTE(...) __attribute__((deprecated(__VA_ARGS__)))


from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef enum:
    A # = 0,
  ctypedef int32_t DeprecatedEnum;

  cdef enum:
    B # = 0,
  ctypedef int32_t DeprecatedEnumWithNote;

  ctypedef struct DeprecatedStruct:
    int32_t a;

  ctypedef struct DeprecatedStructWithNote:
    int32_t a;

  void deprecated_without_note();

  void deprecated_without_bracket();

  void deprecated_with_note();

  void deprecated_with_note_and_since();

  void deprecated_with_note_which_requires_to_be_escaped();

  void dummy(DeprecatedEnum a,
             DeprecatedEnumWithNote b,
             DeprecatedStruct c,
             DeprecatedStructWithNote d);
