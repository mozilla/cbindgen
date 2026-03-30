#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  float x;
  float y;
  float w;
  float h;
} Rect;

typedef struct {
  uint8_t r;
  uint8_t g;
  uint8_t b;
  uint8_t a;
} Color;

enum DisplayItem_Tag
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Fill,
  Image,
  ClearScreen,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum DisplayItem_Tag DisplayItem_Tag;
#else
typedef uint8_t DisplayItem_Tag;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

typedef struct {
  DisplayItem_Tag tag;
  Rect _0;
  Color _1;
} Fill_Body;

typedef struct {
  DisplayItem_Tag tag;
  uint32_t id;
  Rect bounds;
} Image_Body;

typedef union {
  DisplayItem_Tag tag;
  Fill_Body fill;
  Image_Body image;
} DisplayItem;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

bool push_item(DisplayItem item);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
