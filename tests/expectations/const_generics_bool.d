module cbindgen;

@nogc nothrow @safe:

alias Str = const char*;

struct HashTable(K, V, bool IS_MAP) {
  @disable this();
  ulong num_buckets;
  ulong capacity;
  ubyte *occupied;
  K *keys;
  V *vals;
}

alias MySet = HashTable!(Str, char, false);

alias SetCallback = void function(Str key);

alias MapCallback = void function(Str key, ulong val);

extern(C) {

MySet *new_set();

void set_for_each(const MySet *set, SetCallback callback);

HashTable!(Str, ulong, true) *new_map();

void map_for_each(const HashTable!(Str, ulong, true) *map, MapCallback callback);

}  // extern(C)
