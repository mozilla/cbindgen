#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

#define PREFIX_LEN 42

typedef int32_t PREFIX_NamedLenArray[PREFIX_LEN];

typedef int32_t PREFIX_ValuedLenArray[42];

void root(PREFIX_NamedLenArray x, PREFIX_ValuedLenArray y);
