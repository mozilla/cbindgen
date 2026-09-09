#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct A A;

typedef struct B B;

typedef struct C C;

typedef struct F F;

typedef struct H H;

typedef struct O O;

typedef struct P P;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(P p, P *pp, uint16_t t, uint16_t *tp, O o, O *op);

void bytes_and_nums(uint8_t *b, uint16_t *n, uint8_t **bp, uint16_t **np);

void other(A x, B y, C z, F f, H h);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
