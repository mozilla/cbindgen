#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

/// Reachable from an exported function, so it is emitted even though its crate
/// does not contribute top-level items.
struct UsedDepStruct {
  uint32_t x;
};

/// Not used by any exported item, but declared by the binding crate.
struct UnusedLocalStruct {
  uint32_t z;
};

extern "C" {

uint32_t get_x(const UsedDepStruct *used);

}  // extern "C"
