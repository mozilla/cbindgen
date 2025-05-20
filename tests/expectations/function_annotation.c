#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#define TEST_MACRO
#ifndef _In_
#define _In_
#endif


TEST_MACRO void TEST_MACRO root(_In_ const uint64_t *_Nonnull input,
                                uint64_t input_size) TEST_MACRO;
