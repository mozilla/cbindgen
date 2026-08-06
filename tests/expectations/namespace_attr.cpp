#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

/// A function without namespace attribute - uses global namespace
void global_function();


namespace ffi {

/// A function with a single namespace
void ffi_function();

}  // namespace ffi


namespace ffi {
namespace inner {

/// A function with nested namespaces using :: separator
void nested_function(const char *a);

/// Another function with the same namespace to test grouping
void another_nested_function();

}  // namespace inner
}  // namespace ffi


namespace other {
namespace ns {

/// A function with a different nested namespace
void other_namespace_function();

}  // namespace ns
}  // namespace other

}  // extern "C"
