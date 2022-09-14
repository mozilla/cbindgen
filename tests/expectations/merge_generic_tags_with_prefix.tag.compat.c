#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum COption_Tag {
  COption_Some,
  COption_None,
};

struct COption_u8 {
  enum COption_Tag tag;
  union {
    struct {
      uint8_t some;
    };
  };
};



struct COption_u32 {
  enum COption_Tag tag;
  union {
    struct {
      uint32_t some;
    };
  };
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct COption_u8 a, struct COption_u32 b);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
