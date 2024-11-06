#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct MyStruct;

template<typename T = void>
struct NotReprC;

template<typename T = void>
struct RefCell;

using Foo = NotReprC<RefCell<int32_t>>;

struct MyStruct {
  int32_t number;
};

extern "C" {

void root(const Foo *a, const MyStruct *with_cell);

}  // extern "C"
