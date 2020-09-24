#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

template<typename T = void>
struct MaybeUninit;

template<typename T = void>
struct NotReprC;

using Foo = NotReprC<MaybeUninit<const int32_t*>>;

struct MyStruct {
  const int32_t *number;
};

extern "C" {

void root(const Foo *a, const MyStruct *with_maybe_uninit);

} // extern "C"
