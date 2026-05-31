#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr static const int64_t CONST_TEST_1 = 50;

constexpr static const uint64_t CONST_TEST_2 = 100;

struct EnumTest {
  enum class Tag {
    enum_var1,
    enum_var2,
    enum_var3,
  };

  struct enum_var2_Body {
    uint8_t _0[5];
  };

  struct enum_var3_Body {
    bool a;
    uint64_t b;
  };

  Tag tag;
  union {
    enum_var2_Body enum_var2;
    enum_var3_Body enum_var3;
  };
};

struct AnotherStruct {
  uint8_t a;
  int64_t b;
  bool c[36];
};

struct UnionTest {
  enum class Tag {
    union_var1,
    union_var2,
    union_var3,
  };

  struct union_var2_Body {
    uint8_t _0[5];
  };

  struct union_var3_Body {
    bool a;
    uint64_t b;
  };

  Tag tag;
  union {
    union_var2_Body union_var2;
    union_var3_Body union_var3;
  };
};

using typedef_test = int64_t;

extern "C" {

extern const uint64_t STATIC_TEST_1[5];

extern const bool STATIC_TEST_2;

void test_enum(EnumTest enum_);

void test_struct_gen(AnotherStruct struct_);

void test_union(UnionTest union_);

void test_typedefs(typedef_test typedef_);

int64_t test_fn(const int64_t *struct_);

void test_func_ptr(bool (*fn_ptr)(uint64_t, bool));

void test_raw_ptr(const bool *a, uint64_t *b);

}  // extern "C"
