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
    public NativeLong len;

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
    public NativeLong len;

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
    public NativeLong len;

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
    public NativeLong len;

  }


  void init_parens_parser(Parser_40__41ByReference p, ByteByReference buf, NativeLong len);

  void destroy_parens_parser(Parser_40__41ByReference p);

  void init_braces_parser(Parser_123__125ByReference p, ByteByReference buf, NativeLong len);

}