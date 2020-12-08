#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

static const uint8_t SOME_NUMBER = 20;

template<typename N = void>
struct StructWithConstGeneric;

extern "C" {

void root(const StructWithConstGeneric<SOME_NUMBER> *a);

} // extern "C"
