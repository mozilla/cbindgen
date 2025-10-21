#if 0
''' '
#endif

// FIXME: Mis-generated in C mode with enum.prefix_with_name = false, and in
// C++ mode with it set to true...
#if defined(__cplusplus) && !defined(CBINDGEN_CPP_COMPAT)


from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  const uintptr_t AXIS_SHIFT # = 3

  const uintptr_t SELF_WM_SHIFT # = 6

  const uint8_t SELF_WM # = (1 << 6)

  # Specifies which tracks(s) on the axis that the position-area span occupies.
  # Represented as 3 bits: start, center, end track.
  cdef enum:
    # First track
    Start # = 1,
    # First and center.
    SpanStart # = 3,
    # Last track.
    End # = 4,
    # Last and center.
    SpanEnd # = 6,
    # Center track.
    Center # = 2,
    # All tracks
    SpanAll # = 7,
  ctypedef uint8_t PositionAreaTrack;

  # A three-bit value that represents the axis in which position-area operates on.
  # Represented as 3 bits: axis type (physical or logical), direction type (physical or logical),
  # axis value.
  cdef enum:
    Horizontal # = 0,
    Vertical # = 1,
    X # = 2,
    Y # = 3,
    Inline # = 6,
    Block # = 7,
  ctypedef uint8_t PositionAreaAxis;

  # Possible values for the `position-area` property's keywords.
  # Represented by [0z xxx yyy], where z means "self wm resolution", xxx is the type (as in
  # PositionAreaAxis and yyy is the PositionAreaTrack
  # https://drafts.csswg.org/css-anchor-position-1/#propdef-position-area
  cdef enum:
    None # = 0,
    Center # = <uint8_t>PositionAreaTrack_Center,
    SpanAll # = <uint8_t>PositionAreaTrack_SpanAll,
    Start # = <uint8_t>PositionAreaTrack_Start,
    End # = <uint8_t>PositionAreaTrack_End,
    SpanStart # = <uint8_t>PositionAreaTrack_SpanStart,
    SpanEnd # = <uint8_t>PositionAreaTrack_SpanEnd,
    Top # = ((<uint8_t>PositionAreaAxis_Vertical << AXIS_SHIFT) | <uint8_t>PositionAreaTrack_Start),
    Bottom # = ((<uint8_t>PositionAreaAxis_Vertical << AXIS_SHIFT) | <uint8_t>PositionAreaTrack_End),
  ctypedef uint8_t PositionAreaKeyword;

  void root(PositionAreaKeyword, PositionAreaTrack, PositionAreaAxis);

#endif
#if 0
' '''
#endif
