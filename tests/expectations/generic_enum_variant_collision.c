#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  int32_t x;
} Payload;

typedef enum {
  ResultA_Payload_Ok_Payload,
  ResultA_Payload_Err_Payload,
} ResultA_Payload_Tag;

typedef struct {
  ResultA_Payload_Tag tag;
  union {
    struct {
      Payload ok;
    };
    struct {
      void *err;
    };
  };
} ResultA_Payload;

typedef enum {
  ResultB_Payload_Ok_Payload,
  ResultB_Payload_Err_Payload,
} ResultB_Payload_Tag;

typedef struct {
  ResultB_Payload_Tag tag;
  union {
    struct {
      Payload ok;
    };
    struct {
      void *err;
    };
  };
} ResultB_Payload;

void use_a(ResultA_Payload _a);

void use_b(ResultB_Payload _b);
