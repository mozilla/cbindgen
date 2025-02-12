#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  int32_t dummy_field;
} DummyStruct;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

DummyStruct new_dummy(void);

DummyStruct new_dummy_param(int32_t dummy_field);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
