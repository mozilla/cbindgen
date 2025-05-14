#include <stdint.h>

typedef uint64_t Option_Foo;


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  Option_Foo foo;
} Bar;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(Bar f);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
