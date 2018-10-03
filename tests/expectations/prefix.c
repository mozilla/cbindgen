#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

#define PREFIX_LEN 42

typedef int32_t PREFIX_NamedLenArray[PREFIX_LEN];

typedef int32_t PREFIX_ValuedLenArray[42];

enum PREFIX_AbsoluteFontWeight_Tag {
  Weight,
  Normal,
  Bold,
};
typedef uint8_t PREFIX_AbsoluteFontWeight_Tag;

typedef struct {
  PREFIX_AbsoluteFontWeight_Tag tag;
  float _0;
} Weight_Body;

typedef union {
  PREFIX_AbsoluteFontWeight_Tag tag;
  Weight_Body weight;
} PREFIX_AbsoluteFontWeight;

void root(PREFIX_NamedLenArray x, PREFIX_ValuedLenArray y, PREFIX_AbsoluteFontWeight z);
