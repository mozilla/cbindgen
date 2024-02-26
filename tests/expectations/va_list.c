#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

int32_t va_list_test(va_list ap);

int32_t my_snprintf(char *buf, size_t n, const char *format, va_list ap);

int32_t my_vsnprintf(char *buf, size_t n, const char *format, va_list ap);
