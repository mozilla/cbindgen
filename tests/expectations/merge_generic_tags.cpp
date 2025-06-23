#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T>
struct COption {
  enum class Tag {
    Some,
    None,
  };struct Some_Body {
    T _0;
  };



  Tag tag;
  union {
    Some_Body some;
  };
};

extern "C" {

void root(COption<uint8_t> a, COption<uint32_t> b);

}  // extern "C"
