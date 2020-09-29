#ifdef __cplusplus
template <typename T>
using Box = T*;
#endif


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

template<typename T = void>
struct NotReprC;

template<typename T = void>
struct Option;

using Foo = NotReprC<Box<int32_t>>;

struct MyStruct {
  Box<int32_t> number;
};

extern "C" {

void drop_box(Box<int32_t> x);

void drop_box_opt(Option<Box<int32_t>> x);

void root(const Foo *a, const MyStruct *with_box);

} // extern "C"
