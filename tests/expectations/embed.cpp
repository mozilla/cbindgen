#include <cstdarg>
#include <cstdint>
#include <cstdlib>

#ifdef MY_PLATFORM
#define API __declspec(dllimport)
#else
#define API
#endif

extern "C" {

API void root(uint32_t x);

} // extern "C"
