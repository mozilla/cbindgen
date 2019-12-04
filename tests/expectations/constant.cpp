#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

static const uint32_t DELIMITER = ':';

static const uint32_t EQUID = L'\u10083';

static const int32_t FOO = 10;

static const uint32_t HEART = L'\u2764';

static const uint32_t LEFTCURLY = '{';

static const int8_t NEG_ONE = -1;

static const uint32_t NEWLINE = '\n';

static const int8_t POS_ONE = 1;

static const uint32_t QUOTE = '\'';

static const int64_t SHIFT = 3;

static const uint32_t TAB = '\t';

static const int64_t XBOOL = 1;

static const int64_t XFALSE = ((0 << SHIFT) | XBOOL);

static const int64_t XTRUE = (1 << (SHIFT | XBOOL));

static const float ZOM = 3.14;

struct Foo {
  int32_t x[FOO];
};

extern "C" {

void root(Foo x);

} // extern "C"
