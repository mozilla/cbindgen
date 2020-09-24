#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

struct OneFieldIgnored {
  int32_t x;
  int32_t z;
};

struct AllFieldsIgnored {

};

extern "C" {

void no_ignore_root(OneFieldIgnored one, AllFieldsIgnored all);

} // extern "C"
