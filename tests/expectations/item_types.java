import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  class OnlyThisShouldBeGenerated extends IntegerType {
    public OnlyThisShouldBeGenerated() {
      super(4, true);
    }

    public OnlyThisShouldBeGenerated(long value) {
      super(4, value, true);
    }

    public OnlyThisShouldBeGenerated(Pointer p) {
      this(p.getInt(0));
    }
    public static final OnlyThisShouldBeGenerated Foo = new OnlyThisShouldBeGenerated(1);
    public static final OnlyThisShouldBeGenerated Bar = new OnlyThisShouldBeGenerated(2);

  }

  class OnlyThisShouldBeGeneratedByReference extends ByReference {
    public OnlyThisShouldBeGeneratedByReference() {
      super(4);
    }

    public OnlyThisShouldBeGeneratedByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public OnlyThisShouldBeGenerated getValue() {
      Pointer p = getPointer();
      return new OnlyThisShouldBeGenerated(p.getInt(0));
    }

    public void setValue(OnlyThisShouldBeGenerated value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }


}