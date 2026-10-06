#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  uint32_t int_field;
} MyStruct_1Z;

typedef struct {
  uint32_t x;
} S_u32__1Z;

typedef enum {
  Left_1Z__u8,
  Right_1Z__u8,
} Either_1Z__u8_Tag;

typedef struct {
  Either_1Z__u8_Tag tag;
  union {
    struct {

    };
    struct {
      uint8_t right;
    };
  };
} Either_1Z__u8;

MyStruct_1Z my_test(void);

S_u32__1Z f(void);

Either_1Z__u8 returns_either(void);
