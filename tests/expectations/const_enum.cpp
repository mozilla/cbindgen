#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class FillRule : uint8_t {
  Nonzero,
  Evenodd,
};

struct Style {
  FillRule rule;
};
constexpr static const FillRule Style_DEFAULT_RULE = FillRule::Nonzero;
constexpr static const FillRule Style_ALL_RULES[2] = { FillRule::Nonzero, FillRule::Evenodd, };

constexpr static const FillRule DEFAULT_FILL_RULE = FillRule::Nonzero;

constexpr static const FillRule ALL_FILL_RULES[2] = { FillRule::Nonzero, FillRule::Evenodd, };

extern "C" {

void root(FillRule rule, Style style);

}  // extern "C"
