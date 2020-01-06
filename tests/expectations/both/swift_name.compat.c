#define CF_SWIFT_NAME(_name) __attribute__((swift_name(#_name)))

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Foo Foo;

typedef struct FooRef {
  Foo *ptr;
} FooRef;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

FooRef FooRef_create(void) CF_SWIFT_NAME(FooRef.create());

int32_t FooRef_doThing(FooRef self) /*a comment!*/ CF_SWIFT_NAME(FooRef.doThing(self:));

int32_t FooRef_getBar(FooRef self) CF_SWIFT_NAME(FooRef.getBar(self:));

void FooRef_setBar(FooRef self, int32_t bar) CF_SWIFT_NAME(FooRef.setBar(self:bar:));

void do_the_thing(void) CF_SWIFT_NAME(do_the_thing());

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
