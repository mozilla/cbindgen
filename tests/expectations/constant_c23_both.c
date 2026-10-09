#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

constexpr static const uint8_t SMALL = (uint8_t)(255);

constexpr static const uint64_t LARGE = (uint64_t)(18446744073709551615ull);

constexpr static const int32_t NEGATIVE = (int32_t)(-42);

constexpr static const uint32_t MASK = (uint32_t)(~0u);

constexpr static const uintptr_t ARITHMETIC = (uintptr_t)(((4 + 2) * 3));

constexpr static const uint32_t SHIFT = (uint32_t)((1u << 31));

constexpr static const uint32_t BITWISE = (uint32_t)((5 | 2));

constexpr static const bool ENABLED = (bool)(true);

constexpr static const uint32_t CHARACTER = (uint32_t)('x');

constexpr static const float FLOAT = (float)(0.1);

constexpr static const double DOUBLE = (double)(0.3333333333333333);

constexpr static const double FLOAT_DIVISION = (double)((1.0 / 3.0));

constexpr static const int64_t NESTED_NEGATION = (int64_t)(-~42);

constexpr static const uint32_t BOUND = (uint32_t)(UINT32_MAX);

constexpr static const int32_t SIGNED_MIN = (int32_t)(INT32_MIN);

#define POINTER (const uint8_t*)1

typedef struct Aggregate {
  uint32_t value;
} Aggregate;
constexpr static const uint32_t Aggregate_ASSOCIATED = (uint32_t)(6);

typedef uint32_t Alias;

#define ARRAY { 1, 2, }

#define STRING "text"

#define AGGREGATE (Aggregate){ .value = 4 }

#define ALIAS 5

void use_types(struct Aggregate, Alias);
