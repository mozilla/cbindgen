#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

using MyId = void*;

extern "C" {

void takes_id(MyId id);

void takes_unit_ptr(void *id);

}  // extern "C"
