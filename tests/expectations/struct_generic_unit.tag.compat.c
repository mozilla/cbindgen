#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct MyStruct_c_void {
  uint32_t int_field;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct MyStruct_c_void my_test(void);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
