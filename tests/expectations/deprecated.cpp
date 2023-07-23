#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class [[deprecated]] DeprecatedEnum : int32_t {
  A = 0,
};

struct DeprecatedStruct {
  int32_t a;
};

extern "C" {

[[deprecated]]
void deprecated_without_note();

[[deprecated("This is a note")]]
void deprecated_without_bracket();

[[deprecated("This is a note")]]
void deprecated_with_note();

[[deprecated("This is a note")]]
void deprecated_with_note_and_since();

[[deprecated("This quote \" requires to be quoted, and this [\n] requires to be escaped")]]
void deprecated_with_note_which_requires_to_be_escaped();

void dummy(DeprecatedEnum a, DeprecatedStruct b);

} // extern "C"
