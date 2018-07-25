#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

enum OnlyThisShouldBeGenerated {
  Foo,
  Bar,
};
typedef uint8_t OnlyThisShouldBeGenerated;
