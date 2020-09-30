#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>
#ifndef NO_RETURN_ATTR
  #ifdef __GNUC__
    #define NO_RETURN_ATTR __attribute__ ((noreturn))
  #else // __GNUC__
    #define NO_RETURN_ATTR
  #endif // __GNUC__
#endif // NO_RETURN_ATTR


extern "C" {

void loop_forever() NO_RETURN_ATTR;

uint8_t normal_return();

} // extern "C"
