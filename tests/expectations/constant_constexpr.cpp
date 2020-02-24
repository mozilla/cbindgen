#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

constexpr float CONSTANT_FLOAT32 = 312.292;

constexpr int64_t CONSTANT_I64 = 216;

constexpr uint32_t DELIMITER = ':';

constexpr uint32_t LEFTCURLY = '{';

struct Foo {
  int32_t x;
};

static const Foo SomeFoo = Foo{ /* .x = */ 99 };
