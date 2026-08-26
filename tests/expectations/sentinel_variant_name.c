#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum A
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  A_A1,
  A_A2,
  A_A3,
  /**
   * Must be last for serialization purposes
   */
  A_Terminator,
};
#if __STDC_VERSION__ >= 202311L
typedef enum A A;
#else
typedef uint8_t A;
#endif // __STDC_VERSION__ >= 202311L

enum B
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  B_B1,
  B_B2,
  B_B3,
  /**
   * Must be last for serialization purposes
   */
  B_Last,
};
#if __STDC_VERSION__ >= 202311L
typedef enum B B;
#else
typedef uint8_t B;
#endif // __STDC_VERSION__ >= 202311L

void root(A a, B b);
