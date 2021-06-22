#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef uint32_t Handle_File;

struct Node {
  Handle_File file;
  Handle_File maybe_file;
};

void root(const struct Node *node);
