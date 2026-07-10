#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr static const char STRING[] = "string";

constexpr static const char C_STRING[] = "c string";

constexpr static const char EMPTY[] = "";

constexpr static const char WITH_QUOTES[] = "a \"quoted\" word";

constexpr static const char WITH_BACKSLASH[] = "back\\slash";

constexpr static const char WITH_CONTROLS[] = "tab\tnewline\nreturn\r";

constexpr static const char OTHER_CONTROLS[] = "\x01\x0b\x0c\x7f";

constexpr static const char TRIGRAPHS[] = "what\?\?!";

constexpr static const char INTERIOR_NUL[] = "a\x00" "b";

constexpr static const char NON_UTF8[] = "\xff\xfe";

constexpr static const char NON_ASCII[] = "caf\xc3\xa9";

constexpr static const char HEX_ADJACENT[] = "\xc3" "a";

constexpr static const char HEX_ADJACENT_UPPER[] = "\xc3" "F";

constexpr static const char HEX_NON_ADJACENT[] = "\xc3z";

constexpr static const char *ARRAY[2] = { "first", "second", };

/// A documented string constant.
constexpr static const char DOCUMENTED[] = "documented";

extern "C" {

void root();

}  // extern "C"
