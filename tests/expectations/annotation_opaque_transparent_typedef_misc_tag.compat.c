#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct A;

struct B;

struct C;

struct F;

struct H;

struct O;

struct P;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct P p, struct P *pp, uint16_t t, uint16_t *tp, struct O o, struct O *op);

void bytes_and_nums(uint8_t *b, uint16_t *n, uint8_t **bp, uint16_t **np);

void other(struct A x, struct B y, struct C z, struct F f, struct H h);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
