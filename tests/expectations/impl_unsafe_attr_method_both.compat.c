#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct DummyStruct {
  int32_t dummy_field;
} DummyStruct;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct DummyStruct new_dummy(void);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
