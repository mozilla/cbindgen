#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct A;

struct B;

struct C;

struct F;

struct H;

struct P;

struct T;

typedef uint8_t *BytePtr;

typedef uint16_t *NumberPtr;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct A x, struct B y, struct C z, struct F f, struct H h, struct P p, struct T t);

void bytes_and_nums(BytePtr b, NumberPtr n, BytePtr *bp, NumberPtr *np);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
