#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define CONST_TEST_1 50

#define CONST_TEST_2 100

typedef enum EnumTest_Tag {
  enum_var1,
  enum_var2,
  enum_var3,
} EnumTest_Tag;

typedef struct enum_var3_Body {
  bool a;
  uint64_t b;
} enum_var3_Body;

typedef struct EnumTest {
  EnumTest_Tag tag;
  union {
    struct {
      uint8_t enum_var2[5];
    };
    enum_var3_Body enum_var3;
  };
} EnumTest;

typedef struct AnotherStruct {
  uint8_t a;
  int64_t b;
  bool c[36];
} AnotherStruct;

typedef enum UnionTest_Tag {
  union_var1,
  union_var2,
  union_var3,
} UnionTest_Tag;

typedef struct union_var3_Body {
  bool a;
  uint64_t b;
} union_var3_Body;

typedef struct UnionTest {
  UnionTest_Tag tag;
  union {
    struct {
      uint8_t union_var2[5];
    };
    union_var3_Body union_var3;
  };
} UnionTest;

typedef int64_t typedef_test;

extern const uint64_t STATIC_TEST_1[5];

extern const bool STATIC_TEST_2;

void test_enum(struct EnumTest enum_);

void test_struct_gen(struct AnotherStruct struct_);

void test_union(struct UnionTest union_);

void test_typedefs(typedef_test typedef_);

int64_t test_fn(const int64_t *struct_);

void test_func_ptr(bool (*fn_ptr)(uint64_t, bool));

void test_raw_ptr(const bool *a, uint64_t *b);
