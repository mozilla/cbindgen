#define CF_SWIFT_NAME(_name) __attribute__((swift_name(#_name)))

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Foo;

struct FooRef {
  struct Foo *ptr;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct FooRef FooRef_create(void) CF_SWIFT_NAME(FooRef.create());

int32_t FooRef_doThing(struct FooRef self) /*a comment!*/ CF_SWIFT_NAME(FooRef.doThing(self:));

int32_t FooRef_getBar(struct FooRef self) CF_SWIFT_NAME(FooRef.getBar(self:));

void FooRef_setBar(struct FooRef self, int32_t bar) CF_SWIFT_NAME(FooRef.setBar(self:bar:));

void do_the_thing(void) CF_SWIFT_NAME(do_the_thing());

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
