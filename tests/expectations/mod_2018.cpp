#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

static const uint8_t EXPORT_ME_TOO = 42;

struct ExportMe {
  uint64_t val;
};

struct ExportMe2 {
  uint64_t val;
};

extern "C" {

void export_me(ExportMe *val);

void export_me_2(ExportMe2*);

void from_really_nested_mod();

} // extern "C"
