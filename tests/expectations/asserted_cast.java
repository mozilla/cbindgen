import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class I extends PointerType {
    public I() {
      super(null);
    }
    public I(Pointer p) {
      super(p);
    }
  }

  class IByReference extends I {
    public IByReference() {
      super(null);
    }
    public IByReference(Pointer p) {
      super(p);
    }
  }


  class H extends IntegerType {
    public H() {
      super(4, true);
    }

    public H(long value) {
      super(4, value, true);
    }

    public H(Pointer p) {
      this(p.getInt(0));
    }
    public static final H H_Foo = new H(1);
    public static final H H_Bar = new H(2);
    public static final H H_Baz = new H(3);

  }

  class HByReference extends ByReference {
    public HByReference() {
      super(4);
    }

    public HByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public H getValue() {
      Pointer p = getPointer();
      return new H(p.getInt(0));
    }

    public void setValue(H value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class J extends IntegerType {
    public J() {
      super(4, true);
    }

    public J(long value) {
      super(4, value, true);
    }

    public J(Pointer p) {
      this(p.getInt(0));
    }
    public static final J J_Foo = new J(1);
    public static final J J_Bar = new J(2);
    public static final J J_Baz = new J(3);

  }

  class JByReference extends ByReference {
    public JByReference() {
      super(4);
    }

    public JByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public J getValue() {
      Pointer p = getPointer();
      return new J(p.getInt(0));
    }

    public void setValue(J value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class K extends IntegerType {
    public K() {
      super(4, true);
    }

    public K(long value) {
      super(4, value, true);
    }

    public K(Pointer p) {
      this(p.getInt(0));
    }
    public static final K K_Foo = new K(1);
    public static final K K_Bar = new K(2);
    public static final K K_Baz = new K(3);

  }

  class KByReference extends ByReference {
    public KByReference() {
      super(4);
    }

    public KByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public K getValue() {
      Pointer p = getPointer();
      return new K(p.getInt(0));
    }

    public void setValue(K value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }


  void foo(H h, I i, J j, K k);

}