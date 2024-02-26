#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum {
  Some,
  None,
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

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(COption_u8 a, COption_u32 b);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
