#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef MY_PLATFORM
#define API __declspec(dllimport)
#else
#define API
#endif

API void root(uint32_t x);
