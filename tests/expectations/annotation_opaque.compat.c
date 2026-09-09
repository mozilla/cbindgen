#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct A A;

typedef struct B B;

typedef struct C C;

typedef struct F F;

typedef struct H H;

typedef struct P P;

typedef struct T T;

typedef uint8_t *BytePtr;

typedef uint16_t *NumberPtr;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(A x, B y, C z, F f, H h, P p, T t);

void bytes_and_nums(BytePtr b, NumberPtr n, BytePtr *bp, NumberPtr *np);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
