module cbindgen;

@nogc nothrow @safe:

enum TITLE_SIZE = 80;

alias CArrayString(ulong CAP) = byte[CAP] ;

struct Book {
  @disable this();
  CArrayString!(TITLE_SIZE) title;
  CArrayString!(40) author;
}

extern(C) {

void root(Book *a);

}  // extern(C)
