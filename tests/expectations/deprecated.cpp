#define DEPRECATED_FUNC __attribute__((deprecated))
#define DEPRECATED_STRUCT __attribute__((deprecated))
#define DEPRECATED_ENUM __attribute__((deprecated))
#define DEPRECATED_FUNC_WITH_NOTE(...) __attribute__((deprecated(__VA_ARGS__)))
#define DEPRECATED_STRUCT_WITH_NOTE(...) __attribute__((deprecated(__VA_ARGS__)))
#define DEPRECATED_ENUM_WITH_NOTE(...) __attribute__((deprecated(__VA_ARGS__)))


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class DEPRECATED_STRUCT DeprecatedEnum : int32_t {
  A = 0,
};

struct DEPRECATED_STRUCT DeprecatedStruct {
  int32_t a;
};

extern "C" {

DEPRECATED_FUNC void deprecated_without_note();

DEPRECATED_FUNC_WITH_NOTE("This is a note") void deprecated_without_bracket();

DEPRECATED_FUNC_WITH_NOTE("This is a note") void deprecated_with_note();

DEPRECATED_FUNC_WITH_NOTE("This is a note") void deprecated_with_note_and_since();

DEPRECATED_FUNC_WITH_NOTE("This quote \" requires to be quoted, and this [\n] requires to be escaped")
void deprecated_with_note_which_requires_to_be_escaped();

void dummy(DeprecatedEnum a, DeprecatedStruct b);

} // extern "C"
