#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Foo {
  enum class Tag {
    A,
  };

  Tag tag;
  union {
    struct {
      union {
        float a[20];
      };
    };
  };

  static Foo A(const float (&a)[20]) {
    Foo result;
    for (int i = 0; i < 20; i++) {
      ::new (&result.a[i]) (float)(a[i]);
    }
    result.tag = Tag::A;
    return result;
  }

  bool IsA() const {
    return tag == Tag::A;
  }
};

extern "C" {

void root(Foo a);

} // extern "C"
