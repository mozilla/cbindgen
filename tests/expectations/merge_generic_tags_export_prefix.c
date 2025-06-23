#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum {
  PREFIX_C_OPTION_SOME,
  PREFIX_C_OPTION_NONE,
} Prefix_COption_Tag;

typedef enum {
  PREFIX_C_RESULT_OK,
  PREFIX_C_RESULT_ERR,
} Prefix_CResult_Tag;



typedef struct {
  Prefix_COption_Tag tag;
  union {
    struct {
      uint32_t some;
    };
  };
} Prefix_COption_u32;

typedef struct {
  int32_t code;
  const uint8_t *message;
} Prefix_ErrorInfo;



typedef struct {
  Prefix_CResult_Tag tag;
  union {
    struct {
      uint32_t ok;
    };
    struct {
      Prefix_ErrorInfo err;
    };
  };
} Prefix_CResult_u32__ErrorInfo;



typedef struct {
  Prefix_COption_Tag tag;
  union {
    struct {
      const uint8_t *some;
    };
  };
} Prefix_COption______u8;



typedef struct {
  Prefix_CResult_Tag tag;
  union {
    struct {
      const uint8_t *ok;
    };
    struct {
      int32_t err;
    };
  };
} Prefix_CResult______u8__i32;



typedef struct {
  Prefix_COption_Tag tag;
  union {
    struct {
      int32_t some;
    };
  };
} Prefix_COption_i32;

Prefix_COption_u32 process_result(Prefix_CResult_u32__ErrorInfo r);

Prefix_COption______u8 process_str_result(Prefix_CResult______u8__i32 r);

Prefix_COption_i32 get_option_int(void);

Prefix_COption______u8 get_option_str(void);
