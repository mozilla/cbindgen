import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"bytes"})
  class Inner_1 extends Structure implements Structure.ByValue {
    public Inner_1() {
      super();
    }

    public Inner_1(Pointer p) {
      super(p);
    }

    public byte[] bytes;

  }

  @Structure.FieldOrder({"bytes"})
  class Inner_1ByReference extends Structure implements Structure.ByReference {
    public Inner_1ByReference() {
      super();
    }

    public Inner_1ByReference(Pointer p) {
      super(p);
    }

    public byte[] bytes;

  }



  @Structure.FieldOrder({"inner"})
  class Outer_1 extends Structure implements Structure.ByValue {
    public Outer_1() {
      super();
    }

    public Outer_1(Pointer p) {
      super(p);
    }

    public Inner_1 inner;

  }

  @Structure.FieldOrder({"inner"})
  class Outer_1ByReference extends Structure implements Structure.ByReference {
    public Outer_1ByReference() {
      super();
    }

    public Outer_1ByReference(Pointer p) {
      super(p);
    }

    public Inner_1 inner;

  }



  @Structure.FieldOrder({"bytes"})
  class Inner_2 extends Structure implements Structure.ByValue {
    public Inner_2() {
      super();
    }

    public Inner_2(Pointer p) {
      super(p);
    }

    public byte[] bytes;

  }

  @Structure.FieldOrder({"bytes"})
  class Inner_2ByReference extends Structure implements Structure.ByReference {
    public Inner_2ByReference() {
      super();
    }

    public Inner_2ByReference(Pointer p) {
      super(p);
    }

    public byte[] bytes;

  }



  @Structure.FieldOrder({"inner"})
  class Outer_2 extends Structure implements Structure.ByValue {
    public Outer_2() {
      super();
    }

    public Outer_2(Pointer p) {
      super(p);
    }

    public Inner_2 inner;

  }

  @Structure.FieldOrder({"inner"})
  class Outer_2ByReference extends Structure implements Structure.ByReference {
    public Outer_2ByReference() {
      super();
    }

    public Outer_2ByReference(Pointer p) {
      super(p);
    }

    public Inner_2 inner;

  }


  Outer_1 one();

  Outer_2 two();

}