#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct A;

struct B;

struct C;

struct F;

struct H;

struct O;

struct P;

extern "C" {

void root(P p, P *pp, uint16_t t, uint16_t *tp, O o, O *op);

void bytes_and_nums(uint8_t *b, uint16_t *n, uint8_t **bp, uint16_t **np);

void other(A x, B y, C z, F f, H h);

}  // extern "C"
