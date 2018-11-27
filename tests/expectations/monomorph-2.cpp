#include <cstdarg>
#include <cstdint>
#include <cstdlib>

struct A;

struct B;

template<typename T>
struct List {
  T *members;
  uintptr_t count;
};

extern "C" {

void bar(List<B> b);

void foo(List<A> a);

} // extern "C"
