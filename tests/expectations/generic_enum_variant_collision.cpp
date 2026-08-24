#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Payload {
  int32_t x;
};

template<typename T>
struct ResultA {
  enum class Tag {
    ResultA_Ok,
    ResultA_Err,
  };

  struct ResultA_Ok_Body {
    T _0;
  };

  struct ResultA_Err_Body {
    void *_0;
  };

  Tag tag;
  union {
    ResultA_Ok_Body ok;
    ResultA_Err_Body err;
  };
};

template<typename T>
struct ResultB {
  enum class Tag {
    ResultB_Ok,
    ResultB_Err,
  };

  struct ResultB_Ok_Body {
    T _0;
  };

  struct ResultB_Err_Body {
    void *_0;
  };

  Tag tag;
  union {
    ResultB_Ok_Body ok;
    ResultB_Err_Body err;
  };
};

extern "C" {

void use_a(ResultA<Payload> _a);

void use_b(ResultB<Payload> _b);

}  // extern "C"
