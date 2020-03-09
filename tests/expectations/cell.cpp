#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

template<typename T>
struct NotReprC;

template<typename T>
struct RefCell;

using Foo = NotReprC<RefCell<int32_t>>;

struct MyStruct {
  int32_t number;
};

extern "C" {

void root(const Foo *a, const MyStruct *with_cell);

} // extern "C"
