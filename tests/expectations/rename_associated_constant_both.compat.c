#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Foo {

} Foo;
#define FOO_GA 10
#define FOO_ZO 3.14

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct Foo x);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
