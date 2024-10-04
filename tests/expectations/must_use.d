module cbindgen;

@nogc nothrow @safe:

enum MaybeOwnedPtr_Tag : ubyte {
  Owned,
  None,
}

struct MaybeOwnedPtr {
  MaybeOwnedPtr_Tag tag;
  union {
    struct {
      T *owned;
    };
  };
}

struct OwnedPtr(T) {
  @disable this();
  T *ptr;
}

extern(C) {

MaybeOwnedPtr!(int) maybe_consume(OwnedPtr!(int) input);

}  // extern(C)
