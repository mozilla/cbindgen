import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class StyleOnlyThisShouldBeGenerated extends IntegerType {
    public StyleOnlyThisShouldBeGenerated() {
      super(4);
    }

    public StyleOnlyThisShouldBeGenerated(long value) {
      super(4, value);
    }

    public StyleOnlyThisShouldBeGenerated(Pointer p) {
      this(p.getInt(0));
    }
    public static final StyleOnlyThisShouldBeGenerated Foo = new StyleOnlyThisShouldBeGenerated(1);
    public static final StyleOnlyThisShouldBeGenerated Bar = new StyleOnlyThisShouldBeGenerated(2);

  }

  class StyleOnlyThisShouldBeGeneratedByReference extends ByReference {
    public StyleOnlyThisShouldBeGeneratedByReference() {
      super(4);
    }

    public StyleOnlyThisShouldBeGeneratedByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public StyleOnlyThisShouldBeGenerated getValue() {
      return new StyleOnlyThisShouldBeGenerated(getPointer().getInt(0));
    }

    public void setValue(StyleOnlyThisShouldBeGenerated value) {
      getPointer().setInt(0, value.intValue());
    }

  }

}