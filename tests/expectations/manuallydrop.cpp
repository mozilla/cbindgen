#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

template<typename T = void>
struct ManuallyDrop;

template<typename T = void>
struct NotReprC;

struct Point {
  int32_t x;
  int32_t y;
};

using Foo = NotReprC<ManuallyDrop<Point>>;

struct MyStruct {
  Point point;
};

extern "C" {

void root(const Foo *a, const MyStruct *with_manual_drop);

void take(Point with_manual_drop);

} // extern "C"
