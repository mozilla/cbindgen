#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum COption_Tag {
  Some,
  None,
} COption_Tag;

typedef struct COption_u8 {
  COption_Tag tag;
  union {
    struct {
      uint8_t some;
    };
  };
} COption_u8;



typedef struct COption_u32 {
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

void root(struct COption_u8 a, struct COption_u32 b);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
