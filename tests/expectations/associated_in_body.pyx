from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  # Constants shared by multiple CSS Box Alignment properties
  #
  # These constants match Gecko's `NS_STYLE_ALIGN_*` constants.
  ctypedef struct StyleAlignFlags:
    uint8_t bits;
  # 'auto'
  const StyleAlignFlags StyleAlignFlags_AUTO # = <StyleAlignFlags>{ <uint8_t>0 }
  # 'normal'
  const StyleAlignFlags StyleAlignFlags_NORMAL # = <StyleAlignFlags>{ <uint8_t>1 }
  # 'start'
  const StyleAlignFlags StyleAlignFlags_START # = <StyleAlignFlags>{ <uint8_t>(1 << 1) }
  # 'end'
  const StyleAlignFlags StyleAlignFlags_END # = <StyleAlignFlags>{ <uint8_t>(1 << 2) }
  const StyleAlignFlags StyleAlignFlags_ALIAS # = <StyleAlignFlags>{ <uint8_t>(StyleAlignFlags_END).bits }
  # 'flex-start'
  const StyleAlignFlags StyleAlignFlags_FLEX_START # = <StyleAlignFlags>{ <uint8_t>(1 << 3) }
  const StyleAlignFlags StyleAlignFlags_MIXED # = <StyleAlignFlags>{ <uint8_t>(((1 << 4) | (StyleAlignFlags_FLEX_START).bits) | (StyleAlignFlags_END).bits) }
  const StyleAlignFlags StyleAlignFlags_MIXED_SELF # = <StyleAlignFlags>{ <uint8_t>(((1 << 5) | (StyleAlignFlags_FLEX_START).bits) | (StyleAlignFlags_END).bits) }

  void root(StyleAlignFlags flags);
