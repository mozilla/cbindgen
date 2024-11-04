#if 0
DEF DEFINE_FEATURE_1 = 0
DEF DEFINE_FEATURE_2 = 0
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Foo_u16 {
  uint16_t x;
} Foo_u16;

#if defined(DEFINE_FEATURE_1)
typedef struct Foo_u16 FooConditional_u16;
#endif

typedef struct Foo_i16 {
  int16_t x;
} Foo_i16;

typedef struct Foo_i8 {
  int8_t x;
} Foo_i8;

typedef struct NotReturnValue_i32 {
  int32_t x;
} NotReturnValue_i32;

typedef struct FooField {
  struct Foo_i8 (*f)(void);
  void (*g)(struct NotReturnValue_i32);
} FooField;

typedef struct Bar_i16__i16 {
  int16_t p;
  int16_t q;
} Bar_i16__i16;

typedef struct Bar_i8__i32 {
  int8_t p;
  int32_t q;
} Bar_i8__i32;

typedef struct Bar_i8__i32 IntBar_i32;

typedef struct Bar_i8__bool {
  int8_t p;
  bool q;
} Bar_i8__bool;

typedef struct Bar_i8__bool IntBar_bool;

typedef IntBar_bool IntBoolBar;

typedef struct Foo_i32 {
  int32_t x;
} Foo_i32;

typedef struct Foo_i32 WrapFoo_i32;

typedef struct Bar_bool__bool {
  bool p;
  bool q;
} Bar_bool__bool;

typedef struct Bar_bool__bool BoolBoolBar;

typedef BoolBoolBar WrapBoolBoolBar;

typedef struct Foo_bool {
  bool x;
} Foo_bool;

typedef int8_t WrapNonZeroInt;

typedef struct Foo_i64 {
  int64_t x;
} Foo_i64;

typedef struct Foo_i64 Transparent;

typedef struct Foo_u8 {
  uint8_t x;
} Foo_u8;

#if defined(DEFINE_FEATURE_2)
FooConditional_u16 double_feature(void);
#endif

int32_t fnA(void);

int16_t fnB(void);

struct Foo_i16 fnE(void);

void fnF(struct FooField f);

struct Bar_i16__i16 fnG(void);

IntBar_i32 fnH(void);

IntBoolBar fnI(void);

WrapFoo_i32 fnJ(void);

WrapBoolBoolBar fnK(void);

struct Foo_bool fnL(void);

WrapNonZeroInt fnM(void);

Transparent fnN(void);

#if defined(DEFINE_FEATURE_1)
struct Foo_u8 fnO(void);
#endif

struct Foo_u8 fnP(void);
