#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void deprecated_without_note(void);

void deprecated_with_value(void);

void deprecated_with_note(void);

void deprecated_with_note_and_since(void);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
