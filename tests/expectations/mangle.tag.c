#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum Bar {
  BarSome,
  BarThing,
};

struct FooU8 {
  uint8_t a;
};

typedef struct FooU8 Boo;

enum Dog_Tag {
  DogWoof,
};

struct Dog {
  enum Dog_Tag tag;
  union {
    struct {
      struct FooU8 woof;
    };
  };
};

void root(Boo x, enum Bar y, struct Dog z);
