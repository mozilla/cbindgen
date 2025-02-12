#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct DummyStruct {
  int32_t dummy_field;
};

extern "C" {

DummyStruct new_dummy();

}  // extern "C"
