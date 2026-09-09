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

void root(struct P p, struct P *pp, uint16_t t, uint16_t *tp, struct O o, struct O *op);

void bytes_and_nums(uint8_t *b, uint16_t *n, uint8_t **bp, uint16_t **np);

void other(struct A x, struct B y, struct C z, struct F f, struct H h);
