import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  /* Unsupported literal for constant TITLE_SIZE */


  class CArrayString_TITLE_SIZE extends PointerType {
    public CArrayString_TITLE_SIZE() {
      super(null);
    }
    public CArrayString_TITLE_SIZE(Pointer p) {
      super(p);
    }
  }

  class CArrayString_TITLE_SIZEByReference extends CArrayString_TITLE_SIZE {
    public CArrayString_TITLE_SIZEByReference() {
      super(null);
    }
    public CArrayString_TITLE_SIZEByReference(Pointer p) {
      super(p);
    }
  }

  class CArrayString_40 extends PointerType {
    public CArrayString_40() {
      super(null);
    }
    public CArrayString_40(Pointer p) {
      super(p);
    }
  }

  class CArrayString_40ByReference extends CArrayString_40 {
    public CArrayString_40ByReference() {
      super(null);
    }
    public CArrayString_40ByReference(Pointer p) {
      super(p);
    }
  }


  @Structure.FieldOrder({"title", "author"})
  class Book extends Structure implements Structure.ByValue {
    public Book() {
      super();
    }

    public Book(Pointer p) {
      super(p);
    }

    public CArrayString_TITLE_SIZE title;
    public CArrayString_40 author;

  }

  @Structure.FieldOrder({"title", "author"})
  class BookByReference extends Structure implements Structure.ByReference {
    public BookByReference() {
      super();
    }

    public BookByReference(Pointer p) {
      super(p);
    }

    public CArrayString_TITLE_SIZE title;
    public CArrayString_40 author;

  }


  void root(BookByReference a);

}