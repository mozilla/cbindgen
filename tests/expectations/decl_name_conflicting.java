import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class BindingType extends IntegerType {
    public BindingType() {
      super(4);
    }

    public BindingType(long value) {
      super(4, value);
    }

    public BindingType(Pointer p) {
      this(p.getInt(0));
    }
    public static final BindingType Buffer = new BindingType(0);
    public static final BindingType NotBuffer = new BindingType(1);

  }

  class BindingTypeByReference extends ByReference {
    public BindingTypeByReference() {
      super(4);
    }

    public BindingTypeByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public BindingType getValue() {
      return new BindingType(getPointer().getInt(0));
    }

    public void setValue(BindingType value) {
      getPointer().setInt(0, value.intValue());
    }

  }


  @Structure.FieldOrder({"ty"})
  class BindGroupLayoutEntry extends Structure implements Structure.ByValue {
    public BindGroupLayoutEntry() {
      super();
    }

    public BindGroupLayoutEntry(Pointer p) {
      super(p);
    }

    public BindingType ty;

  }

  @Structure.FieldOrder({"ty"})
  class BindGroupLayoutEntryByReference extends Structure implements Structure.ByReference {
    public BindGroupLayoutEntryByReference() {
      super();
    }

    public BindGroupLayoutEntryByReference(Pointer p) {
      super(p);
    }

    public BindingType ty;

  }


  void root(BindGroupLayoutEntry entry);

}