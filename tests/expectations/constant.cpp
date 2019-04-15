#include <cstdarg>
#include <cstdint>
#include <cstdlib>

static const wchar_t DELIMITER = ':';

static const int32_t FOO = 10;

static const wchar_t HEART = '\u{2764}';

static const wchar_t NEWLINE = '\n';

static const wchar_t QUOTE = '\'';

static const wchar_t TAB = '\t';

static const float ZOM = 3.14;

struct Foo {
  int32_t x[FOO];
};

extern "C" {

void root(Foo x);

} // extern "C"
