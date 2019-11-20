#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <uchar.h>

struct dep_struct {
  uint32_t x;
  double y;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

uint32_t get_x(const struct dep_struct *dep_struct);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
