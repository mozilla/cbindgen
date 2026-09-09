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

struct P;

struct T;

using BytePtr = uint8_t*;

using NumberPtr = uint16_t*;

extern "C" {

void root(A x, B y, C z, F f, H h, P p, T t);

void bytes_and_nums(BytePtr b, NumberPtr n, BytePtr *bp, NumberPtr *np);

}  // extern "C"
