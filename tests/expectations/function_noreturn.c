#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#ifndef NO_RETURN_ATTR
  #ifdef __GNUC__
    #define NO_RETURN_ATTR __attribute__ ((noreturn))
  #else // __GNUC__
    #define NO_RETURN_ATTR
  #endif // __GNUC__
#endif // NO_RETURN_ATTR


typedef struct {
  void (*f)(uintptr_t, uintptr_t) NO_RETURN_ATTR;
} Example;

void loop_forever(void) NO_RETURN_ATTR;

uint8_t normal_return(Example arg, void (*other)(uint8_t) NO_RETURN_ATTR);
