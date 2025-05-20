
import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class HashMap_i32__i32__BuildHasherDefault_DefaultHasher extends PointerType {
    public HashMap_i32__i32__BuildHasherDefault_DefaultHasher() {
      super(null);
    }
    public HashMap_i32__i32__BuildHasherDefault_DefaultHasher(Pointer p) {
      super(p);
    }
  }

  class HashMap_i32__i32__BuildHasherDefault_DefaultHasherByReference extends HashMap_i32__i32__BuildHasherDefault_DefaultHasher {
    public HashMap_i32__i32__BuildHasherDefault_DefaultHasherByReference() {
      super(null);
    }
    public HashMap_i32__i32__BuildHasherDefault_DefaultHasherByReference(Pointer p) {
      super(p);
    }
  }

  class Result_Foo extends PointerType {
    public Result_Foo() {
      super(null);
    }
    public Result_Foo(Pointer p) {
      super(p);
    }
  }

  class Result_FooByReference extends Result_Foo {
    public Result_FooByReference() {
      super(null);
    }
    public Result_FooByReference(Pointer p) {
      super(p);
    }
  }


  /**
   * Fast hash map used internally.
   */
  class FastHashMap_i32__i32 extends HashMap_i32__i32__BuildHasherDefault_DefaultHasher {
    public FastHashMap_i32__i32() {
      super();
    }
    public FastHashMap_i32__i32(Pointer p) {
      super(p);
    }
  }


  /**
   * Fast hash map used internally.
   */
  class FastHashMap_i32__i32ByReference extends HashMap_i32__i32__BuildHasherDefault_DefaultHasherByReference {
    public FastHashMap_i32__i32ByReference() {
      super();
    }
    public FastHashMap_i32__i32ByReference(Pointer p) {
      super(p);
    }
  }

  class Foo extends FastHashMap_i32__i32 {
    public Foo() {
      super();
    }
    public Foo(Pointer p) {
      super(p);
    }
  }

  class FooByReference extends FastHashMap_i32__i32ByReference {
    public FooByReference() {
      super();
    }
    public FooByReference(Pointer p) {
      super(p);
    }
  }

  class Bar extends Result_Foo {
    public Bar() {
      super();
    }
    public Bar(Pointer p) {
      super(p);
    }
  }

  class BarByReference extends Result_FooByReference {
    public BarByReference() {
      super();
    }
    public BarByReference(Pointer p) {
      super(p);
    }
  }

  void root(FooByReference a, BarByReference b);

}