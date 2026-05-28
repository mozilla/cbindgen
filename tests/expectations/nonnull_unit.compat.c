#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef void *MyId;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void takes_id(MyId id);

void takes_unit_ptr(void *id);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
