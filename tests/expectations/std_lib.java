import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class Option_i32 extends PointerType {
    public Option_i32() {
      super(null);
    }
    public Option_i32(Pointer p) {
      super(p);
    }
  }

  class Option_i32ByReference extends Option_i32 {
    public Option_i32ByReference() {
      super(null);
    }
    public Option_i32ByReference(Pointer p) {
      super(p);
    }
  }

  class Result_i32__String extends PointerType {
    public Result_i32__String() {
      super(null);
    }
    public Result_i32__String(Pointer p) {
      super(p);
    }
  }

  class Result_i32__StringByReference extends Result_i32__String {
    public Result_i32__StringByReference() {
      super(null);
    }
    public Result_i32__StringByReference(Pointer p) {
      super(p);
    }
  }

  class Vec_String extends PointerType {
    public Vec_String() {
      super(null);
    }
    public Vec_String(Pointer p) {
      super(p);
    }
  }

  class Vec_StringByReference extends Vec_String {
    public Vec_StringByReference() {
      super(null);
    }
    public Vec_StringByReference(Pointer p) {
      super(p);
    }
  }

  void root(Vec_StringByReference a, Option_i32ByReference b, Result_i32__StringByReference c);

}