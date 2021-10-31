#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum {
  BarSome,
  BarThing,
} Bar;

typedef struct {
  uint8_t a;
} FooU8;

typedef FooU8 Boo;

typedef enum {
  DogWoof,
} Dog_Tag;

typedef struct {
  Dog_Tag tag;
  union {
    struct {
      FooU8 woof;
    };
  };
} Dog;

void root(Boo x, Bar y, Dog z);
