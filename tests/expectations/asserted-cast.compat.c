#define MY_ASSERT(...) do { } while (0)
#define MY_ATTRS __attribute((noinline))


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct I I;

enum H_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  H_Foo,
  H_Bar,
  H_Baz,
};
#ifndef __cplusplus
typedef uint8_t H_Tag;
#endif // __cplusplus

typedef struct {
  int16_t _0;
} H_Foo_Body;

typedef struct {
  uint8_t x;
  int16_t y;
} H_Bar_Body;

typedef struct {
  H_Tag tag;
  union {
    H_Foo_Body foo;
    H_Bar_Body bar;
  };
} H;

enum J_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  J_Foo,
  J_Bar,
  J_Baz,
};
#ifndef __cplusplus
typedef uint8_t J_Tag;
#endif // __cplusplus

typedef struct {
  int16_t _0;
} J_Foo_Body;

typedef struct {
  uint8_t x;
  int16_t y;
} J_Bar_Body;

typedef struct {
  J_Tag tag;
  union {
    J_Foo_Body foo;
    J_Bar_Body bar;
  };
} J;

enum K_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  K_Foo,
  K_Bar,
  K_Baz,
};
#ifndef __cplusplus
typedef uint8_t K_Tag;
#endif // __cplusplus

typedef struct {
  K_Tag tag;
  int16_t _0;
} K_Foo_Body;

typedef struct {
  K_Tag tag;
  uint8_t x;
  int16_t y;
} K_Bar_Body;

typedef union {
  K_Tag tag;
  K_Foo_Body foo;
  K_Bar_Body bar;
} K;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void foo(H h, I i, J j, K k);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
