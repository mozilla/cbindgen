#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr const uint8_t SMALL = 255;

constexpr const uint64_t LARGE = 18446744073709551615ull;

constexpr const int32_t NEGATIVE = -42;

constexpr const uint32_t MASK = ~0u;

constexpr const uintptr_t ARITHMETIC = ((4 + 2) * 3);

constexpr const uint32_t SHIFT = (1u << 31);

constexpr const uint32_t BITWISE = (5 | 2);

constexpr const bool ENABLED = true;

constexpr const uint32_t CHARACTER = 'x';

constexpr const float FLOAT = 0.1;

constexpr const double DOUBLE = 0.3333333333333333;

constexpr const double FLOAT_DIVISION = (1.0 / 3.0);

constexpr const int64_t NESTED_NEGATION = -~42;

constexpr const uint32_t BOUND = UINT32_MAX;

constexpr const int32_t SIGNED_MIN = INT32_MIN;

#define POINTER (const uint8_t*)1

struct Aggregate {
  uint32_t value;
};
constexpr const uint32_t Aggregate_ASSOCIATED = 6;

using Alias = uint32_t;

constexpr const uint8_t ARRAY[2] = { 1, 2, };

constexpr const char STRING[] = "text";

constexpr const Aggregate AGGREGATE = Aggregate{
  /* .value = */ 4
};

constexpr const Alias ALIAS = 5;

extern "C" {

void use_types(Aggregate, Alias);

}  // extern "C"
