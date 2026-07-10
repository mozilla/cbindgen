from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  const char STRING[] # = "string"

  const char C_STRING[] # = "c string"

  const char EMPTY[] # = ""

  const char WITH_QUOTES[] # = "a \"quoted\" word"

  const char WITH_BACKSLASH[] # = "back\\slash"

  const char WITH_CONTROLS[] # = "tab\tnewline\nreturn\r"

  const char OTHER_CONTROLS[] # = "\x01\x0b\x0c\x7f"

  const char TRIGRAPHS[] # = "what\?\?!"

  const char INTERIOR_NUL[] # = "a\x00" "b"

  const char NON_UTF8[] # = "\xff\xfe"

  const char NON_ASCII[] # = "caf\xc3\xa9"

  const char HEX_ADJACENT[] # = "\xc3" "a"

  const char HEX_ADJACENT_UPPER[] # = "\xc3" "F"

  const char HEX_NON_ADJACENT[] # = "\xc3z"

  const char *ARRAY[2] # = [ "first", "second", ]

  # A documented string constant.
  const char DOCUMENTED[] # = "documented"

  void root();
