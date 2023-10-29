#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

int32_t va_list_test(va_list ap);

int32_t my_snprintf(char *buf, size_t n, const char *format, va_list ap);

int32_t my_vsnprintf(char *buf, size_t n, const char *format, va_list ap);

} // extern "C"
