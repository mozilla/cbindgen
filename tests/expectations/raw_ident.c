#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum Enum
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  a,
  b,
};
#if __STDC_VERSION__ >= 202311L
typedef enum Enum Enum;
#else
typedef uint8_t Enum;
#endif // __STDC_VERSION__ >= 202311L

typedef struct {
  Enum field;
} Struct;

extern const Enum STATIC;

void fn(Struct arg);
