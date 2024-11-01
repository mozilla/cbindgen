#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T>
struct Foo {
  T x;
};

template<typename P, typename Q>
struct Bar {
  P p;
  Q q;
};

/// Dummy struct emitted by cbindgen to avoid compiler warnings/errors about
/// return type C linkage for template types returned by value from functions
struct __cbindgen_return_value_monomorphs {
  Foo<bool> field0;
  Bar<int16_t, int16_t> field1;
  Bar<bool, bool> field2;
  Foo<int16_t> field3;
  Foo<int32_t> field4;
  Bar<int8_t, int32_t> field5;
  Foo<int64_t> field6;
  Bar<int8_t, bool> field7;
  Foo<int8_t> field8;
};

template<typename T>
struct NotReturnValue {
  T x;
};

struct FooField {
  Foo<int8_t> (*f)();
  void (*g)(NotReturnValue<int32_t>);
};

template<typename T>
using IntBar = Bar<int8_t, T>;

using IntBoolBar = IntBar<bool>;

template<typename T>
using WrapFoo = Foo<T>;

using BoolBoolBar = Bar<bool, bool>;

using WrapBoolBoolBar = BoolBoolBar;

using WrapNonZeroInt = int8_t;

using Transparent = Foo<int64_t>;

extern "C" {

int32_t fnA();

int16_t fnB();

Foo<int16_t> fnE();

void fnF(FooField f);

Bar<int16_t, int16_t> fnG();

IntBar<int32_t> fnH();

IntBoolBar fnI();

WrapFoo<int32_t> fnJ();

WrapBoolBoolBar fnK();

Foo<bool> fnL();

WrapNonZeroInt fnM();

Transparent fnN();

}  // extern "C"
