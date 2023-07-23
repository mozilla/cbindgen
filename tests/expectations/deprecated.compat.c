#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

[[deprecated]] void deprecated_without_note(void);

[[deprecated("This is a note")]] void deprecated_without_bracket(void);

[[deprecated("This is a note")]] void deprecated_with_note(void);

[[deprecated("This is a note")]] void deprecated_with_note_and_since(void);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
