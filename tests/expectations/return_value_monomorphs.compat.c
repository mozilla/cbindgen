#if 0
DEF DEFINE_FEATURE_1 = 0
DEF DEFINE_FEATURE_2 = 0
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  uint16_t x;
} Foo_u16;

#if defined(DEFINE_FEATURE_1)
typedef Foo_u16 FooConditional_u16;
#endif

typedef struct {
  int16_t x;
} Foo_i16;

typedef struct {
  int8_t x;
} Foo_i8;

typedef struct {
  int32_t x;
} NotReturnValue_i32;

typedef struct {
  Foo_i8 (*f)(void);
  void (*g)(NotReturnValue_i32);
} FooField;

typedef struct {
  int16_t p;
  int16_t q;
} Bar_i16__i16;

typedef struct {
  int8_t p;
  int32_t q;
} Bar_i8__i32;

typedef Bar_i8__i32 IntBar_i32;

typedef struct {
  int8_t p;
  bool q;
} Bar_i8__bool;

typedef Bar_i8__bool IntBar_bool;

typedef IntBar_bool IntBoolBar;

typedef struct {
  int32_t x;
} Foo_i32;

typedef Foo_i32 WrapFoo_i32;

typedef struct {
  bool p;
  bool q;
} Bar_bool__bool;

typedef Bar_bool__bool BoolBoolBar;

typedef BoolBoolBar WrapBoolBoolBar;

typedef struct {
  bool x;
} Foo_bool;

typedef int8_t WrapNonZeroInt;

typedef struct {
  int64_t x;
} Foo_i64;

typedef Foo_i64 Transparent;

typedef struct {
  uint8_t x;
} Foo_u8;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#if defined(DEFINE_FEATURE_2)
FooConditional_u16 double_feature(void);
#endif

int32_t fnA(void);

int16_t fnB(void);

Foo_i16 fnE(void);

void fnF(FooField f);

Bar_i16__i16 fnG(void);

IntBar_i32 fnH(void);

IntBoolBar fnI(void);

WrapFoo_i32 fnJ(void);

WrapBoolBoolBar fnK(void);

Foo_bool fnL(void);

WrapNonZeroInt fnM(void);

Transparent fnN(void);

#if defined(DEFINE_FEATURE_1)
Foo_u8 fnO(void);
#endif

Foo_u8 fnP(void);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
