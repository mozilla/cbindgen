#define CF_SWIFT_NAME(_name) __attribute__((swift_name(#_name)))

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

struct Foo;

struct FooRef {
  Foo *ptr;
};

extern "C" {

FooRef FooRef_create() CF_SWIFT_NAME(FooRef.create());

int32_t FooRef_doThing(FooRef self) /*a comment!*/ CF_SWIFT_NAME(FooRef.doThing(self:));

int32_t FooRef_getBar(FooRef self) CF_SWIFT_NAME(FooRef.getBar(self:));

void FooRef_setBar(FooRef self, int32_t bar) CF_SWIFT_NAME(FooRef.setBar(self:bar:));

void do_the_thing() CF_SWIFT_NAME(do_the_thing());

} // extern "C"
