import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"buf", "len"})
  class Parser_40__41 extends Structure implements Structure.ByValue {
    public Parser_40__41() {
      super();
    }

    public Parser_40__41(Pointer p) {
      super(p);
    }

    public ByteByReference buf;
    public _Size len;

  }

  @Structure.FieldOrder({"buf", "len"})
  class Parser_40__41ByReference extends Structure implements Structure.ByReference {
    public Parser_40__41ByReference() {
      super();
    }

    public Parser_40__41ByReference(Pointer p) {
      super(p);
    }

    public ByteByReference buf;
    public _Size len;

  }



  @Structure.FieldOrder({"buf", "len"})
  class Parser_123__125 extends Structure implements Structure.ByValue {
    public Parser_123__125() {
      super();
    }

    public Parser_123__125(Pointer p) {
      super(p);
    }

    public ByteByReference buf;
    public _Size len;

  }

  @Structure.FieldOrder({"buf", "len"})
  class Parser_123__125ByReference extends Structure implements Structure.ByReference {
    public Parser_123__125ByReference() {
      super();
    }

    public Parser_123__125ByReference(Pointer p) {
      super(p);
    }

    public ByteByReference buf;
    public _Size len;

  }


  void init_parens_parser(Parser_40__41ByReference p, ByteByReference buf, _Size len);

  void destroy_parens_parser(Parser_40__41ByReference p);

  void init_braces_parser(Parser_123__125ByReference p, ByteByReference buf, _Size len);

  class _Size extends IntegerType {
    public _Size() {
      super(Native.POINTER_SIZE, true);
    }

    public _Size(long value) {
      super(Native.POINTER_SIZE, value, true);
    }

    public _Size(Pointer p) {
      this(Native.POINTER_SIZE == 8 ? p.getLong(0) : p.getInt(0));
    }

  }

  class _SizeByReference extends ByReference {
    public _SizeByReference() {
      super(Native.POINTER_SIZE);
    }

    public _SizeByReference(Pointer p) {
      super(Native.POINTER_SIZE);
      setPointer(p);
    }

    public _Size getValue() {
      Pointer p = getPointer();
      return new _Size(Native.POINTER_SIZE == 8 ? p.getLong(0) : p.getInt(0));
    }

    public void setValue(_Size value) {
      Pointer p = getPointer();
      if (Native.POINTER_SIZE == 8) { p.setLong(0, value.longValue()); } else { p.setInt(0, value.intValue()); }
    }

  }

}