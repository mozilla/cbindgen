#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define PREFIX_LEN 42

#define PREFIX_X (42 << 42)

#define PREFIX_Y (PREFIX_X + PREFIX_X)

typedef int32_t PREFIX_NamedLenArray[PREFIX_LEN];

typedef int32_t PREFIX_ValuedLenArray[42];

enum PREFIX_AbsoluteFontWeight_Tag {
  Weight,
  Normal,
  Bold,
};
typedef uint8_t PREFIX_AbsoluteFontWeight_Tag;

typedef struct PREFIX_Weight_Body {
  PREFIX_AbsoluteFontWeight_Tag tag;
  float _0;
} PREFIX_Weight_Body;

typedef union PREFIX_AbsoluteFontWeight {
  PREFIX_AbsoluteFontWeight_Tag tag;
  PREFIX_Weight_Body weight;
} PREFIX_AbsoluteFontWeight;

void root(PREFIX_NamedLenArray x, PREFIX_ValuedLenArray y, PREFIX_AbsoluteFontWeight z);
