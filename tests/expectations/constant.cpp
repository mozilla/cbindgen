#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

static const char32_t DELIMITER = ':';

static const char32_t EQUID = L'\u10083';

static const int32_t FOO = 10;

static const char32_t HEART = L'\u2764';

static const char32_t LEFTCURLY = '{';

static const int8_t NEG_ONE = -1;

static const char32_t NEWLINE = '\n';

static const int8_t POS_ONE = 1;

static const char32_t QUOTE = '\'';

static const char32_t TAB = '\t';

static const float ZOM = 3.14;

struct Foo {
  int32_t x[FOO];
};

extern "C" {

void root(Foo x);

} // extern "C"
