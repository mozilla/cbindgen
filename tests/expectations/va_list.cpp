#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

using VaListFnPtr = int32_t(*)(va_list);

using VaListFnPtr2 = int32_t(*)();

template<typename T>
struct Interface {
  T fn1;
};

extern "C" {

int32_t va_list_test(va_list ap);

int32_t va_list_test2(va_list ap);

void va_list_fn_ptrs(int32_t (*fn1)(va_list),
                     int32_t (*fn2)(),
                     VaListFnPtr fn3,
                     VaListFnPtr2 fn4,
                     Interface<int32_t(*)(va_list)> fn5,
                     Interface<int32_t(*)()> fn6);

}  // extern "C"
