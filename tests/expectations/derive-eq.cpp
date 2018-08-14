#include <cstdint>
#include <cstdlib>

struct Foo {
  bool a;
  int32_t b;

  bool operator==(const Foo& other) const {
    return a == other.a &&
           b == other.b;
  }
  bool operator!=(const Foo& other) const {
    return a != other.a ||
           b != other.b;
  }
};

union Bar {
  enum class Tag : uint8_t {
    Baz,
    Bazz,
    FooNamed,
    FooParen,
  };

  struct Bazz_Body {
    Tag tag;
    Foo named;

    bool operator==(const Bazz_Body& other) const {
      return named == other.named;
    }
    bool operator!=(const Bazz_Body& other) const {
      return named != other.named;
    }
  };

  struct FooNamed_Body {
    Tag tag;
    int32_t different;
    uint32_t fields;

    bool operator==(const FooNamed_Body& other) const {
      return different == other.different &&
             fields == other.fields;
    }
    bool operator!=(const FooNamed_Body& other) const {
      return different != other.different ||
             fields != other.fields;
    }
  };

  struct FooParen_Body {
    Tag tag;
    int32_t _0;
    Foo _1;

    bool operator==(const FooParen_Body& other) const {
      return _0 == other._0 &&
             _1 == other._1;
    }
    bool operator!=(const FooParen_Body& other) const {
      return _0 != other._0 ||
             _1 != other._1;
    }
  };

  struct {
    Tag tag;
  };
  Bazz_Body bazz;
  FooNamed_Body foo_named;
  FooParen_Body foo_paren;

  bool operator==(const Bar& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Bazz: return bazz == other.bazz;
      case Tag::FooNamed: return foo_named == other.foo_named;
      case Tag::FooParen: return foo_paren == other.foo_paren;
      default: return true;
    }
  }

  bool operator!=(const Bar& other) const {
    return !(*this == other);
  }
};

extern "C" {

Foo root(Bar bar);

} // extern "C"
