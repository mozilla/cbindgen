#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum C
#ifdef __cplusplus
  : uint32_t
#endif // __cplusplus
 {
  X = 2,
  Y,
};
#ifndef __cplusplus
typedef uint32_t C;
#endif // __cplusplus

typedef struct {
  int32_t _0;
} A;

typedef struct {
  int32_t x;
  float y;
} B;

typedef struct {
  uint8_t List;
  uintptr_t Of;
  B Things;
} D;

enum F_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Foo,
  Bar,
  Baz,
};
#ifndef __cplusplus
typedef uint8_t F_Tag;
#endif // __cplusplus

typedef struct {
  F_Tag tag;
  int16_t _0;
} Foo_Body;

typedef struct {
  F_Tag tag;
  uint8_t x;
  int16_t y;
} Bar_Body;

typedef union {
  F_Tag tag;
  Foo_Body foo;
  Bar_Body bar;
} F;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(A a, B b, C c, D d, F f);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
