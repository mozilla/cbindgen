#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

void pointer_test(const uint64_t *a);

void print_from_rust();

void unnamed(const uint64_t*);

} // extern "C"
