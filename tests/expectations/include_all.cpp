#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class UnusedEnum : uint8_t {
  A,
  B,
};

struct ExplicitOpaque;

struct OpaqueDependency;

struct UnusedStruct {
  int32_t x;
  float y;
};

using UnusedTransparent = uint32_t;

union UnusedUnion {
  int32_t x;
  float y;
};

using UnusedAlias = int32_t;

struct UsesOpaqueDependency {
  OpaqueDependency *opaque;
};

struct PublicInPrivateModule {
  int32_t x;
};
