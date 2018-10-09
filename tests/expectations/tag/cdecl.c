#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef void (*A)();

typedef void (*B)();

typedef bool (*C)(int32_t, int32_t);

typedef bool (*(*D)(int32_t))(float);

typedef int32_t (*(*E)())[16];

typedef const int32_t *F;

typedef const int32_t *const *G;

typedef int32_t *const *H;

typedef int32_t (*I)[16];

typedef double (**J)(float);

typedef int32_t K[16];

typedef const int32_t *L[16];

typedef bool (*M[16])(int32_t, int32_t);

typedef void (*N[16])(int32_t, int32_t);

typedef void (*P)(int32_t named1st, bool, bool named3rd, int32_t _);

void (*O(void))(void);

void root(A a, B b, C c, D d, E e, F f, G g, H h, I i, J j, K k, L l, M m, N n, P p);
