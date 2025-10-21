#if 0
''' '
#endif

// FIXME: Mis-generated in C mode with enum.prefix_with_name = false, and in
// C++ mode with it set to true...
#if defined(__cplusplus) && !defined(CBINDGEN_CPP_COMPAT)


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define AXIS_SHIFT 3

#define SELF_WM_SHIFT 6

#define SELF_WM (1 << 6)

/**
 * Specifies which tracks(s) on the axis that the position-area span occupies.
 * Represented as 3 bits: start, center, end track.
 */
enum PositionAreaTrack
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  /**
   * First track
   */
  Start = 1,
  /**
   * First and center.
   */
  SpanStart = 3,
  /**
   * Last track.
   */
  End = 4,
  /**
   * Last and center.
   */
  SpanEnd = 6,
  /**
   * Center track.
   */
  Center = 2,
  /**
   * All tracks
   */
  SpanAll = 7,
};
#ifndef __cplusplus
typedef uint8_t PositionAreaTrack;
#endif // __cplusplus

/**
 * A three-bit value that represents the axis in which position-area operates on.
 * Represented as 3 bits: axis type (physical or logical), direction type (physical or logical),
 * axis value.
 */
enum PositionAreaAxis
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Horizontal = 0,
  Vertical = 1,
  X = 2,
  Y = 3,
  Inline = 6,
  Block = 7,
};
#ifndef __cplusplus
typedef uint8_t PositionAreaAxis;
#endif // __cplusplus

/**
 * Possible values for the `position-area` property's keywords.
 * Represented by [0z xxx yyy], where z means "self wm resolution", xxx is the type (as in
 * PositionAreaAxis and yyy is the PositionAreaTrack
 * https://drafts.csswg.org/css-anchor-position-1/#propdef-position-area
 */
enum PositionAreaKeyword
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  None = 0,
  Center = (uint8_t)PositionAreaTrack_Center,
  SpanAll = (uint8_t)PositionAreaTrack_SpanAll,
  Start = (uint8_t)PositionAreaTrack_Start,
  End = (uint8_t)PositionAreaTrack_End,
  SpanStart = (uint8_t)PositionAreaTrack_SpanStart,
  SpanEnd = (uint8_t)PositionAreaTrack_SpanEnd,
  Top = (((uint8_t)PositionAreaAxis_Vertical << AXIS_SHIFT) | (uint8_t)PositionAreaTrack_Start),
  Bottom = (((uint8_t)PositionAreaAxis_Vertical << AXIS_SHIFT) | (uint8_t)PositionAreaTrack_End),
};
#ifndef __cplusplus
typedef uint8_t PositionAreaKeyword;
#endif // __cplusplus

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(PositionAreaKeyword, PositionAreaTrack, PositionAreaAxis);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif
#if 0
' '''
#endif
