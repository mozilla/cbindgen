#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct File;

template<typename T>
using Handle = uint32_t;

struct Node {
  Handle<File> file;
  Handle<File> maybe_file;
};

extern "C" {

void root(const Node *node);

} // extern "C"
