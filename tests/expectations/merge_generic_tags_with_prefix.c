#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum {
  COption_Some,
  COption_None,
} COption_Tag;

typedef struct {
  COption_Tag tag;
  union {
    struct {
      uint8_t some;
    };
  };
} COption_u8;



typedef struct {
  COption_Tag tag;
  union {
    struct {
      uint32_t some;
    };
  };
} COption_u32;

void root(COption_u8 a, COption_u32 b);
