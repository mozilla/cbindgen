#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef uint32_t Handle_File;

typedef struct Node {
  Handle_File file;
  uint32_t maybe_file;
} Node;

void root(const struct Node *node);
