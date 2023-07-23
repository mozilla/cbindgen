#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

[[deprecated]] void deprecated_without_note();

[[deprecated("This is a note")]] void deprecated_without_bracket();

[[deprecated("This is a note")]] void deprecated_with_note();

[[deprecated("This is a note")]] void deprecated_with_note_and_since();

} // extern "C"
