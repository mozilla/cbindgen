#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Opaque_1Z Opaque_1Z;

typedef struct Option_1Z Option_1Z;

typedef struct Option_Option_1Z Option_Option_1Z;

typedef struct Option_u32 Option_u32;

typedef struct Result_1Z__1Z Result_1Z__1Z;

typedef struct Result_1Z__u32 Result_1Z__u32;

typedef struct Result_u32__1Z Result_u32__1Z;

typedef struct {
  uint8_t a[3];
  uint32_t b;
} CG_u8__3;

typedef struct {
  void (*args_always_erased)(void);
  const void *zst_behind_ptr;
  const Opaque_1Z *zst_in_opaque;
  void (*f_returns_zst)(uint32_t arg);
} ValidPositions_1Z;

typedef struct {
  uint32_t err;
} APIError;

typedef struct {
  bool result_good;
  const void *result;
  const APIError *err;
} CResultTempl_1Z__APIError;

typedef CResultTempl_1Z__APIError CResultNoneAPIError;

typedef struct {
  uint32_t x;
} Foo_1Z;

void option_u32(Option_u32 arg);

void option_unit(Option_1Z arg, Option_Option_1Z nested);

void result_zsts(Result_1Z__u32 arg1, Result_u32__1Z arg2, Result_1Z__1Z arg3);

void use_cg(CG_u8__3 x);

void use_valid_positions(ValidPositions_1Z arg);

void CResultNoneAPIError_free(CResultNoneAPIError _res);

Foo_1Z bar(void);
