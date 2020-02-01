#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

#if (defined(PLATFORM_WIN) || defined(M_32))
enum class BarType : uint32_t {
  A,
  B,
  C,
};
#endif

#if (defined(PLATFORM_UNIX) && defined(X11))
enum class FooType : uint32_t {
  A,
  B,
  C,
};
#endif

#if (defined(PLATFORM_UNIX) && defined(X11))
struct FooHandle {
  FooType ty;
  int32_t x;
  float y;
};
#endif

union C {
  enum class Tag : uint8_t {
    C1,
    C2,
#if defined(PLATFORM_WIN)
    C3,
#endif
#if defined(PLATFORM_UNIX)
    C5,
#endif
  };

#if defined(PLATFORM_UNIX)
  struct C5_Body {
    Tag tag;
    int32_t int_;
  };
#endif

  struct {
    Tag tag;
  };
#if defined(PLATFORM_UNIX)
  C5_Body c5;
#endif
};

#if (defined(PLATFORM_WIN) || defined(M_32))
struct BarHandle {
  BarType ty;
  int32_t x;
  float y;
};
#endif

extern "C" {

#if (defined(PLATFORM_UNIX) && defined(X11))
void root(FooHandle a, C c);
#endif

#if (defined(PLATFORM_WIN) || defined(M_32))
void root(BarHandle a, C c);
#endif

} // extern "C"
