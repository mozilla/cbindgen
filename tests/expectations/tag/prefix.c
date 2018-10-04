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

struct PREFIX_Weight_Body {
  PREFIX_AbsoluteFontWeight_Tag tag;
  float _0;
};

union PREFIX_AbsoluteFontWeight {
  enum PREFIX_AbsoluteFontWeight_Tag tag;
  struct PREFIX_Weight_Body weight;
};

void root(PREFIX_NamedLenArray x, PREFIX_ValuedLenArray y, union PREFIX_AbsoluteFontWeight z);
