#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum Bar {
  BarSome,
  BarThing,
} Bar;

typedef struct FooU8 {
  uint8_t a;
} FooU8;

typedef struct FooU8 Boo;

typedef enum Dog_Tag {
  DogWoof,
} Dog_Tag;

typedef struct Dog {
  Dog_Tag tag;
  union {
    struct {
      struct FooU8 woof;
    };
  };
} Dog;

void root(Boo x, enum Bar y, struct Dog z);
