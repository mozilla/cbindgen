#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace global_ns {

extern "C" {

/// A function without namespace attribute - should use global namespace
void uses_global_namespace();

/// Another function without namespace attribute - should use global namespace
void also_uses_global_namespace();


namespace ffi {
namespace bar {

/// A function with per-item namespace - should override global namespace
void uses_item_namespace(const char *a);

}  // namespace bar
}  // namespace ffi

}  // extern "C"

}  // namespace global_ns
