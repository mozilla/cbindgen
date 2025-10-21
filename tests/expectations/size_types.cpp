#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class UE : size_t {
  UV,
};

enum class IE : ptrdiff_t {
  IV,
};

using Usize = size_t;

using Isize = ptrdiff_t;

extern "C" {

void root(Usize, Isize, UE, IE);

}  // extern "C"
