#include <cstdint>
#include <cstdlib>

extern "C" {

struct A;

struct B;

struct List_B {
  B *members;
  size_t count;
};

struct List_A {
  A *members;
  size_t count;
};

void bar(List_B b);

void foo(List_A a);

} // extern "C"

template<typename T>
struct List;

template<>
struct List<B> : public List_B {

};

template<>
struct List<A> : public List_A {

};
