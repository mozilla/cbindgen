#if 0
DEF DEFINE_FEATURE_1 = 0
DEF DEFINE_FEATURE_2 = 0
#endif


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename P, typename Q>
struct Bar {
  P p;
  Q q;
};

template<typename T>
struct Foo {
  T x;
};

/// Dummy struct emitted by cbindgen to avoid compiler warnings/errors about
/// return type C linkage for template types returned by value from functions
struct __cbindgen_return_value_monomorphs {
  Bar<bool, bool> field0;
  Bar<int8_t, bool> field1;
  Bar<int8_t, int32_t> field2;
  Bar<int16_t, int16_t> field3;
  Foo<bool> field4;
#if defined(DEFINE_FEATURE_1)
  Foo<uint8_t> field5
#endif
  ;
  Foo<uint8_t> field6;
#if (defined(DEFINE_FEATURE_2) && defined(DEFINE_FEATURE_1))
  Foo<uint16_t> field7
#endif
  ;
  Foo<int8_t> field8;
  Foo<int16_t> field9;
  Foo<int32_t> field10;
  Foo<int64_t> field11;
};

#if defined(DEFINE_FEATURE_1)
template<typename T>
using FooConditional = Foo<T>;
#endif

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

#if defined(DEFINE_FEATURE_2)
FooConditional<uint16_t> double_feature();
#endif

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

#if defined(DEFINE_FEATURE_1)
Foo<uint8_t> fnO();
#endif

Foo<uint8_t> fnP();

}  // extern "C"
