#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T = void>
struct Opaque;

template<typename T>
struct MyOption {
  enum class Tag {
    Some,
    None,
  };

  struct Some_Body {
    T _0;
  };

  Tag tag;
  union {
    Some_Body some;
  };
};

template<typename T>
struct MyWrapper {
  T inner;
};

template<typename T>
struct S {
  void (*args_erased)(T arg);
  void (*args_erased_with_npo)(T arg);
  MyOption<void(*)(T arg)> args_erased_with_custom_enum;
  MyWrapper<void(*)(T arg)> args_erased_with_custom_struct;
  const Opaque<void(*)(T arg)> *args_erased_with_opaque;
  void (**args_erased_behind_ptr)(T arg);
  void (*args_erased_in_array[2])(T arg);
  void (*(*args_erased_in_ret)())(T arg);
  T arr_erased[2];
  const T (*arr_erased_behind_ptr)[2];
  const Opaque<T[2]> *arr_erased_in_opaque;
};

template<typename T, typename U>
struct S2 {
  void (*f)(T, U);
  U x;
};

template<typename U>
struct Inner {
  const U *p;
};

template<typename T>
struct Outer {
  Inner<T> plain;
  Inner<T[2]> arr;
};

extern "C" {

void use_S(S<void> arg1, S2<void, uint8_t[4]> arg2, Outer<void> arg3);

}  // extern "C"
