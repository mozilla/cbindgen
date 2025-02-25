#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>
#define TEST_MACRO
#ifndef _In_
#define _In_
#endif


extern "C" {

TEST_MACRO void TEST_MACRO root(_In_ const uint64_t *_Nonnull input,
                                uint64_t input_size) TEST_MACRO;

}  // extern "C"
