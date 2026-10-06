#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Opaque_1Z;

struct Option_1Z;

struct Option_Option_1Z;

struct Option_u32;

struct Result_1Z__1Z;

struct Result_1Z__u32;

struct Result_u32__1Z;

struct CG_u8__3 {
  uint8_t a[3];
  uint32_t b;
};

struct ValidPositions_1Z {
  void (*args_always_erased)(void);
  const void *zst_behind_ptr;
  const struct Opaque_1Z *zst_in_opaque;
  void (*f_returns_zst)(uint32_t arg);
};

struct APIError {
  uint32_t err;
};

struct CResultTempl_1Z__APIError {
  bool result_good;
  const void *result;
  const struct APIError *err;
};

typedef struct CResultTempl_1Z__APIError CResultNoneAPIError;

struct Foo_1Z {
  uint32_t x;
};

void option_u32(struct Option_u32 arg);

void option_unit(struct Option_1Z arg, struct Option_Option_1Z nested);

void result_zsts(struct Result_1Z__u32 arg1, struct Result_u32__1Z arg2, struct Result_1Z__1Z arg3);

void use_cg(struct CG_u8__3 x);

void use_valid_positions(struct ValidPositions_1Z arg);

void CResultNoneAPIError_free(CResultNoneAPIError _res);

struct Foo_1Z bar(void);
