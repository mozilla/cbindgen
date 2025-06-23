#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T>
struct Prefix_COption {
  enum class Tag {
    PREFIX_C_OPTION_SOME,
    PREFIX_C_OPTION_NONE,
  };struct Prefix_COption_Prefix_Some_Body {
    T _0;
  };



  Tag tag;
  union {
    Prefix_COption_Prefix_Some_Body SOME;
  };
};

struct Prefix_ErrorInfo {
  int32_t code;
  const uint8_t *message;
};

template<typename T, typename E>
struct Prefix_CResult {
  enum class Tag {
    PREFIX_C_RESULT_OK,
    PREFIX_C_RESULT_ERR,
  };struct Prefix_CResult_Prefix_Ok_Body {
    T _0;
  };

  struct Prefix_CResult_Prefix_Err_Body {
    E _0;
  };



  Tag tag;
  union {
    Prefix_CResult_Prefix_Ok_Body OK;
    Prefix_CResult_Prefix_Err_Body ERR;
  };
};

extern "C" {

Prefix_COption<uint32_t> process_result(Prefix_CResult<uint32_t, Prefix_ErrorInfo> r);

Prefix_COption<const uint8_t*> process_str_result(Prefix_CResult<const uint8_t*, int32_t> r);

Prefix_COption<int32_t> get_option_int();

Prefix_COption<const uint8_t*> get_option_str();

}  // extern "C"
