#ifdef CBINDGEN_CYTHON
from libc.stdint cimport uint64_t
ctypedef uint64_t Option_Foo
#else
#include <stdint.h>
typedef uint64_t Option_Foo;
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Bar {
  Option_Foo foo;
};

void root(struct Bar f);
