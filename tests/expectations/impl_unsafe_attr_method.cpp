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

DummyStruct new_dummy_param(int32_t dummy_field);

}  // extern "C"
