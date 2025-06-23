#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum Prefix_COption_Tag {
  PREFIX_C_OPTION_SOME,
  PREFIX_C_OPTION_NONE,
};

enum Prefix_CResult_Tag {
  PREFIX_C_RESULT_OK,
  PREFIX_C_RESULT_ERR,
};



struct Prefix_COption_u32 {
  enum Prefix_COption_Tag tag;
  union {
    struct {
      uint32_t some;
    };
  };
};

struct Prefix_ErrorInfo {
  int32_t code;
  const uint8_t *message;
};



struct Prefix_CResult_u32__ErrorInfo {
  enum Prefix_CResult_Tag tag;
  union {
    struct {
      uint32_t ok;
    };
    struct {
      struct Prefix_ErrorInfo err;
    };
  };
};



struct Prefix_COption______u8 {
  enum Prefix_COption_Tag tag;
  union {
    struct {
      const uint8_t *some;
    };
  };
};



struct Prefix_CResult______u8__i32 {
  enum Prefix_CResult_Tag tag;
  union {
    struct {
      const uint8_t *ok;
    };
    struct {
      int32_t err;
    };
  };
};



struct Prefix_COption_i32 {
  enum Prefix_COption_Tag tag;
  union {
    struct {
      int32_t some;
    };
  };
};

struct Prefix_COption_u32 process_result(struct Prefix_CResult_u32__ErrorInfo r);

struct Prefix_COption______u8 process_str_result(struct Prefix_CResult______u8__i32 r);

struct Prefix_COption_i32 get_option_int(void);

struct Prefix_COption______u8 get_option_str(void);
