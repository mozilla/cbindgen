#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Payload {
  int32_t x;
};

enum ResultA_Payload_Tag {
  ResultA_Payload_Ok_Payload,
  ResultA_Payload_Err_Payload,
};

struct ResultA_Payload {
  enum ResultA_Payload_Tag tag;
  union {
    struct {
      struct Payload ok;
    };
    struct {
      void *err;
    };
  };
};

enum ResultB_Payload_Tag {
  ResultB_Payload_Ok_Payload,
  ResultB_Payload_Err_Payload,
};

struct ResultB_Payload {
  enum ResultB_Payload_Tag tag;
  union {
    struct {
      struct Payload ok;
    };
    struct {
      void *err;
    };
  };
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void use_a(struct ResultA_Payload _a);

void use_b(struct ResultB_Payload _b);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
