#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

void deprecated_without_note();

void deprecated_with_value();

void deprecated_with_note();

void deprecated_with_note_and_since();

} // extern "C"
